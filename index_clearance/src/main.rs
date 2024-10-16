
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
    
    // Index clear 의 대상이 되는 인덱스들 정보
    //let clear_index_info: Vec<>

    let mut handlers: Vec<MainHandler<IndexClearServicePub<EsRepositoryPub>>> = Vec::new();
    
    for cluster in es_infos_vec {
        let metirc_service = IndexClearServicePub::new(cluster);
        let maind_handler = MainHandler::new(metirc_service);
        handlers.push(maind_handler);
    }
    
    
    
}