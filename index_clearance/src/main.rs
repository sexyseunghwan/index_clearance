/*
Author      : Seunghwan Shin 
Create date : 2024-10-21 
Description : Elasticsearch 특정 인덱스를 삭제해주는 함수.
    
History     : 2024-10-21 Seunghwan Shin       # [v.1.0.0] first create
              2024-10-27 Seunghwan Shin       # [v.1.1.0] elastic 에 security 적용되지 않는 경우라도 프로그램 동작하도록 변경
              2024-11-00 Seunghwan Shin       # [v.1.2.0] 
                                                1) smtp 통신을 통해서 인덱스 삭제 알림 수행.
                                                2) 서버관리 정책으로 인하여 스케쥴러 등록이 제한 -> 프로그램 자체를 스케줄러로 수정.
*/ 

mod common;
use common::*;

mod models;

mod util_modules;
use util_modules::logger_utils::*;
use util_modules::io_utils::*;

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
    
    let schedule = load_schedule_config();
    let mut upcoming = schedule.upcoming(chrono::Utc);
    
    loop {

        let next = match upcoming.next() {
            Some(next) => next,
            None => {
                error!("[Error][main()] Failed to execute schedule");
                continue
            }
        };

        let now = chrono::Utc::now();

        if next > now {
            
            let duration_until_next = match (next - now).to_std() {
                Ok(duration_until_next) => duration_until_next,
                Err(e) => {
                    error!("[Error][main()] Failed to calculate 'duration_until_next': {:?}", e);
                    continue
                }
            };
            
            let sleep_until_time = Instant::now() + duration_until_next;
            sleep_until(sleep_until_time).await;
        }

        schedule_task().await;
    }
       
}


#[doc = "메인함수 - 스케쥴링에 따른다."]
async fn schedule_task() {

    /* Elasticsearch DB 커넥션 정보 */ 
    let es_infos_vec: Vec<EsRepositoryPub> = match initialize_db_clients("./datas/server_info.json") {
        Ok(es_infos_vec) => es_infos_vec,
        Err(e) => {
            error!("[Error][main()] Cannot find json file: {:?}", e);
            panic!("{:?}", e)
        }
    };
    
    /* 의존주입 핸들러 */ 
    let mut handlers: Vec<MainHandler<IndexClearServicePub<EsRepositoryPub>>> = Vec::new();
    
    for cluster in es_infos_vec {
        
        let metirc_service = match IndexClearServicePub::new(cluster) {
            Ok(metirc_service) => metirc_service,
            Err(e) => {
                error!("{:?}", e);
                continue
            }
        };
        
        let main_handler = MainHandler::new(metirc_service);
        handlers.push(main_handler);
    }
    
    
    /* 
        Handler 를 통한 Async 작업 
        각 Elasticsearch Cluster 마다 작업을 진행한다.
        Elasticsearch cluster 가 여러개 있을수 있으므로.
    */ 
    let futures = handlers.iter().map(|handler| {
        async move {                
            handler.task_set().await
        }
    });
    
    
    /* 작업결과 */ 
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