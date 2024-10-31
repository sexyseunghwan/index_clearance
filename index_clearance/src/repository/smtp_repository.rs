use crate::common::*;

use crate::models::SmtpJson::*;
use crate::models::ReceiverEmailList::*;
use crate::models::EmailStruct::*;

use crate::util_modules::io_utils::*;


#[doc = "smtp 통신 객체를 초기화해주는 함수"]
pub fn initialize_smtp_clients(smtp_info_path: &str, email_receiver_info: &str) -> Result<SmtpRepositoryPub, anyhow::Error> {

    let smtp_info_json: SmtpJson = read_json_from_file::<SmtpJson>(smtp_info_path)?;
    let receiver_email_list: ReceiverEmailList =  read_json_from_file::<ReceiverEmailList>(email_receiver_info)?;
    let smtp_repo = 
        SmtpRepositoryPub::new(smtp_info_json, receiver_email_list);
    
    Ok(smtp_repo)
}

#[async_trait]
pub trait SmtpRepository {
    async fn send_message_to_receiver_html(&self, email_id: &str, subject: &str, html_content: &str) -> Result<(), anyhow::Error>;
    async fn send_message_to_receivers(&self, send_email_form: &Vec<EmailStruct>, cluster_name: &str) -> Result<(), anyhow::Error>;
} 


#[derive(Serialize, Deserialize, Debug, Getters, new)]
#[getset(get = "pub")]
pub struct SmtpRepositoryPub {
    smtp_info_json: SmtpJson,
    receiver_email_list: ReceiverEmailList
}


#[async_trait]
impl SmtpRepository for SmtpRepositoryPub {
    
    #[doc = "수신자에게 이메일을 보내주는 함수"]
    async fn send_message_to_receiver_html(&self, email_id: &str, subject: &str, html_content: &str) -> Result<(), anyhow::Error> {

        let email = Message::builder()
            .from(self.smtp_info_json.credential_id.parse().unwrap())
            .to(email_id.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::alternative() 
                    .singlepart(
                        SinglePart::html(html_content.to_string())
                    )
            )?;

        let creds = Credentials::new(
            self.smtp_info_json.credential_id().to_string(), 
            self.smtp_info_json.credential_pw().to_string()
        );
        
        let mailer = 
            AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(self.smtp_info_json.smtp_name().as_str())?
                .credentials(creds)
                .build();
        
        match mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(anyhow!("{:?} : Failed to send email to {} ", e, email_id))
            }
        }
    }  
    
    
    #[doc = "지정된 수신자 모두에게 이메일을 보내주는 함수"]
    async fn send_message_to_receivers(&self, send_email_form: &Vec<EmailStruct>, cluster_name: &str) -> Result<(), anyhow::Error> {
        
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
            self.send_message_to_receiver_html(email_id.as_str(), "[Elasticsearch] Index removed list", &html_content)
        });
        
        let results = join_all(tasks).await;

        for result in results {
            match result {
                Ok(_) => info!("Email sent successfully"),
                Err(e) => error!("[Error][send_message_to_receivers()] Failed to send email: {}", e),
            }
        }
        
        Ok(())
    } 
}