use crate::common::*;

use crate::models::SmtpJson::*;
use crate::models::ReceiverEmailList::*;

use crate::util_modules::io_utils::*;


// #[doc = "Elasticsearch connection 을 싱글톤으로 관리하기 위한 전역 변수."]
// static ELASTICSEARCH_CLIENT: once_lazy<Arc<EsRepositoryPub>> = once_lazy::new(|| {
//     initialize_elastic_clients()
// });


// #[doc = "Function to initialize Elasticsearch connection instances"]
// pub fn initialize_elastic_clients() -> Arc<EsRepositoryPub> {
    
//     let cluster_config: ClusterJson = match read_json_from_file::<ClusterJson>("./datas/server_info.json") {
//         Ok(cluster_config) => cluster_config,
//         Err(err) => {
//             error!("{:?}", err);
//             panic!("{:?}", err)
//         }
//     };
    
//     let es_helper = match EsRepositoryPub::new(
//             cluster_config.hosts.clone(), 
//             &cluster_config.es_id, 
//             &cluster_config.es_pw) {
//                 Ok(es_helper) => es_helper,
//                 Err(err) => {
//                     error!("{:?}", err);
//                     panic!("{:?}", err)
//                 }
//             };
    
//     Arc::new(es_helper)
// }

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
    async fn send_message_to_receivers(&self) -> Result<(), anyhow::Error>;
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
        
        let creds = Credentials::new(self.smtp_info_json.credential_id().to_string(), self.smtp_info_json.credential_pw().to_string());
        
        let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(self.smtp_info_json.smtp_name().as_str())?
            .credentials(creds)
            .build();
        
        mailer.send(email).await?;
        
        Ok(())
    }  
    
    #[doc = "지정된 수신자 모두에게 이메일을 보내주는 함수"]
    async fn send_message_to_receivers(&self) -> Result<(), anyhow::Error> {
        
        let receiver_email_list = self.receiver_email_list.receivers();
        
        let html_template = std::fs::read_to_string("./html/view.html")?;  //HTML 파일 읽기
        //let html_content = html_template.replace("{index_name}", index_name);

        let tasks = receiver_email_list.iter().map(|receiver| {
            let email_id = receiver.email_id();
            self.send_message_to_receiver_html(email_id.as_str(), "[Elasticsearch] Index removed list", &html_template)
        });

        let results = join_all(tasks).await;

        for result in results {
            match result {
                Ok(_) => println!("Email sent successfully"),
                Err(e) => eprintln!("Failed to send email: {}", e),
            }
        }

        
        // for receiver in receiver_email_list {
        //     let email_id = receiver.email_id();
        //     self.send_message_to_receiver_html(email_id.as_str(), "[Elasticsearch] Index removed list", &html_template).await?;
        // }

        Ok(())
    } 
}