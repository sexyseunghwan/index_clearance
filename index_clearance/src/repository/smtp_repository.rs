use crate::common::*;

use crate::models::email_struct::*;
use crate::models::receiver_email_list::*;
use crate::models::smtp_config::*;
use crate::models::system_config::*;

use crate::util_modules::io_utils::*;

#[doc = "전역 SMTP 통신 인스턴스를 선언"]
static SMTP_REPO: once_lazy<Arc<SmtpRepositoryPub>> = once_lazy::new(|| initialize_smtp_clients());

#[doc = "smtp 통신 객체를 초기화해주는 함수"]
pub fn initialize_smtp_clients() -> Arc<SmtpRepositoryPub> {
    let smtp_config: Arc<SmtpConfig> = get_smtp_config_info();

    let receiver_email_list: ReceiverEmailList =
        match read_toml_from_file::<ReceiverEmailList>(EMAIL_RECEIVER_INFO) {
            Ok(receiver_email_list) => receiver_email_list,
            Err(e) => {
                error!(
                    "[Error][initialize_smtp_clients()] Failed to object '{}' {:?}",
                    EMAIL_RECEIVER_INFO, e
                );
                panic!("{:?}", e)
            }
        };

    Arc::new(SmtpRepositoryPub::new(
        smtp_config.smtp_name().to_string(),
        smtp_config.credential_id().to_string(),
        smtp_config.credential_pw().to_string(),
        receiver_email_list,
    ))
}

#[doc = "SMTP를 Thread-safe 하게 이용하는 함수."]
pub fn get_smtp_repo() -> Arc<SmtpRepositoryPub> {
    Arc::clone(&SMTP_REPO)
}

#[async_trait]
pub trait SmtpRepository {
    async fn send_message_to_receiver_html(
        &self,
        email_id_str: &str,
        subject: &str,
        html_content: &str,
        cluster_name: &str
    ) -> Result<(), anyhow::Error>;
    async fn send_message_to_receivers(
        &self,
        send_email_form: &Vec<EmailStruct>,
        cluster_name: &str,
    ) -> Result<(), anyhow::Error>;
}

#[derive(Serialize, Deserialize, Debug, Getters, new)]
#[getset(get = "pub")]
pub struct SmtpRepositoryPub {
    smtp_name: String,
    credential_id: String,
    credential_pw: String,
    receiver_email_list: ReceiverEmailList,
}

#[async_trait]
impl SmtpRepository for SmtpRepositoryPub {
    #[doc = "수신자에게 이메일을 보내주는 함수"]
    async fn send_message_to_receiver_html(
        &self,
        email_id_str: &str,
        subject: &str,
        html_content: &str,
        cluster_name: &str
    ) -> Result<(), anyhow::Error> {
        let credential_id: Mailbox = self.credential_id.parse()?;
        let email_id: Mailbox = email_id_str.parse()?;

        let email = Message::builder()
            .from(credential_id)
            .to(email_id)
            .subject(subject)
            .multipart(
                MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
            )?;

        let creds = Credentials::new(
            self.credential_id().to_string(),
            self.credential_pw().to_string(),
        );

        let mailer =
            AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(self.smtp_name().as_str())?
                .credentials(creds)
                .build();
        
        match mailer.send(email).await {
            Ok(_) => {
                info!("[{}] Email sent successfully: {}", cluster_name, email_id_str);
                Ok(())
            },
            Err(e) => Err(anyhow!(
                "[{}] {:?} : Failed to send email to {} ",
                cluster_name,
                e,
                email_id_str
            )),
        }
    }

    #[doc = "지정된 수신자 모두에게 이메일을 보내주는 함수"]
    async fn send_message_to_receivers(
        &self,
        send_email_form: &Vec<EmailStruct>,
        cluster_name: &str,
    ) -> Result<(), anyhow::Error> {
        let receiver_email_list = self.receiver_email_list.receivers();

        let html_template = std::fs::read_to_string("./html/view.html")?;
        let mut index_list_html = String::new();

        for email in send_email_form {
            index_list_html.push_str(&email.html_form);
        }

        let html_content = html_template
            .replace("{cluster_name}", cluster_name)
            .replace("{index_list}", &index_list_html);

        /* Not Async */
        // for receiver in receiver_email_list {
        //     let email_id = receiver.email_id();
        //     self.send_message_to_receiver_html(email_id.as_str(), "[Elasticsearch] Index removed list", &html_content).await?;
        // }
        
        /* ASYNC TASK */
        let tasks = receiver_email_list.iter().map(|receiver| {
            let email_id = receiver.email_id();
            self.send_message_to_receiver_html(
                email_id.as_str(),
                "[Elasticsearch] Log Index removed list",
                &html_content,
                cluster_name
            )
        });

        let results = join_all(tasks).await;

        for result in results {
            match result {
                Ok(_) => (),
                Err(e) => error!(
                    "[Error][send_message_to_receivers()] Failed to send email: {}",
                    e
                ),
            }
        }
        
        Ok(())
    }
}
