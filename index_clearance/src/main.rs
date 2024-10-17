
mod common;
use common::*;

mod models;

mod util_modules;
use util_modules::logger_utils::*;

mod repository;
use crate::repository::es_repository::*;

mod handler;
use crate::handler::main_handler::*;

mod service;
use service::index_clear_service::*;

#[tokio::main]
async fn main() {

    set_global_logger();
    info!("Index Schedule Program Start");
    
    // Elasticsearch DB 커넥션 정보
    let es_infos_vec: Vec<EsRepositoryPub> = match initialize_db_clients("./datas/server_info.json") {
        Ok(es_infos_vec) => es_infos_vec,
        Err(e) => {
            error!("[Error][main()] Cannot find json file: {:?}", e);
            panic!("{:?}", e)
        }
    };
    
    // 의존주입 핸들러
    let mut handlers: Vec<MainHandler<IndexClearServicePub<EsRepositoryPub>>> = Vec::new();
    
    for cluster in es_infos_vec {
        let metirc_service = IndexClearServicePub::new(cluster);
        let main_handler = MainHandler::new(metirc_service);
        handlers.push(main_handler);
    }
    

    // Handler 를 통한 Async 작업
    let futures = handlers.iter().map(|handler| {
        async move {                
            handler.task_set().await
        }
    });
    
    // 작업결과
    let results = join_all(futures).await;
    
    for result in results {
        match result {
            Ok(_) => {
                info!("Index Schedule Program processed successfully");
            }
            Err(e) => {
                error!("[Error][main()] Error processing : {:?}", e);
            }
        }
    }
    
}