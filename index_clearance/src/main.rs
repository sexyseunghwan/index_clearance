/*
Author      : Seunghwan Shin 
Create date : 2024-10-21 
Description : Elasticsearch 특정 인덱스를 삭제해주는 함수.
    
History     : 2024-10-21 Seunghwan Shin       # first create
              2024-10-29 Seunghwan Shin       # 1) elastic 에 security 적용되지 않는 경우라도 프로그램 동작하도록 변경
                                                2) smtp / telegram 통신을 통해서 인덱스 삭제 알림 수행.
*/ 

mod common;
use core::panic;

use common::*;

mod models;

mod util_modules;
use lettre::AsyncTransport;
use util_modules::logger_utils::*;

mod repository;
use crate::repository::es_repository::*;
use crate::repository::smtp_repository::*;

mod handler;
use crate::handler::main_handler::*;

mod service;
use service::index_clear_service::*;

use models::ReceiverEmailList::*;

use lettre::message::header;
#[tokio::main]
async fn main() {

    set_global_logger();
    info!("Index Schedule Program Start");
    
    
    // let mut file = std::fs::File::open("./html/view.html").expect("Failed to open file");
    // let mut html_content = String::new();
    // file.read_to_string(&mut html_content).expect("Failed to read file");

    let index_name = "my_index";  // 동적으로 변경될 인덱스 이름
    let html_template = std::fs::read_to_string("./html/view.html").unwrap();  // HTML 파일 읽기
    let html_content = html_template.replace("{index_name}", index_name);

    // for elem in receiver_email_list.receivers {
    //     println!("{:?}", elem);
    // }
    
    //let receiver_email_list = ReceiverEmailList::new().unwrap();
    
    // let email = Message::builder()
    //     .from("ssh9308@gmail.com".parse().unwrap())
    //     .to("ssh9308@mediawill.com".parse().unwrap())
    //     .subject("Rust SMTP email test!")
    //     .body(String::from("This is a test email sent from Rust using lettre and Gmail SMTP!"))
    //     .unwrap();
    
    let email = Message::builder()
        .from("ssh9308@gmail.com".parse().unwrap())
        .to("ssh9308@mediawill.com".parse().unwrap())
        .subject("Rust SMTP email test!")
        .multipart(
            MultiPart::alternative()  // 이 부분이 HTML과 일반 텍스트를 모두 포함하도록 설정
                .singlepart(
                    SinglePart::plain(String::from("This is a fallback text email body for email clients that do not support HTML."))
                )
                .singlepart(
                    SinglePart::html(html_content)
                )
        ).unwrap();
    
    
    let creds = Credentials::new("ssh9308@gmail.com".to_string(), "myep aazw gxjo uuxc".to_string());
    
    let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay("smtp.gmail.com").unwrap()
        .credentials(creds)
        .build();

    let res  = mailer.send(email).await.unwrap();
    
    // Send the email
    // match mailer.send(email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => eprintln!("Could not send email: {:?}", e),
    // }
    
    // Elasticsearch DB 커넥션 정보
    // let es_infos_vec: Vec<EsRepositoryPub> = match initialize_db_clients("./datas/server_info.json") {
    //     Ok(es_infos_vec) => es_infos_vec,
    //     Err(e) => {
    //         error!("[Error][main()] Cannot find json file: {:?}", e);
    //         panic!("{:?}", e)
    //     }
    // };
    
    // // 의존주입 핸들러
    // let mut handlers: Vec<MainHandler<IndexClearServicePub<EsRepositoryPub>>> = Vec::new();
    
    // for cluster in es_infos_vec {
        
    //     let metirc_service = match IndexClearServicePub::new(cluster) {
    //         Ok(metirc_service) => metirc_service,
    //         Err(e) => {
    //             error!("{:?}", e);
    //             continue
    //         }
    //     };

    //     let main_handler = MainHandler::new(metirc_service);
    //     handlers.push(main_handler);
    // }
    
    // // Handler 를 통한 Async 작업
    // let futures = handlers.iter().map(|handler| {
    //     async move {                
    //         handler.task_set().await
    //     }
    // });
    
    // // 작업결과
    // let results = join_all(futures).await;
    
    // for result in results {
    //     match result {
    //         Ok(_) => {
    //             info!("Index Schedule Program processed successfully");
    //         }
    //         Err(e) => {
    //             error!("[Error][main()] Error processing : {:?}", e);
    //         }
    //     }
    // }    
}
