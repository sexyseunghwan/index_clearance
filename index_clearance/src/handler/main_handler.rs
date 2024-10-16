use crate::common::*;

use crate::util_modules::io_utils::*;

use crate::service::index_clear_service::*;

use crate::models::ClusterConfig::*;
use crate::models::ClusterJson::*;
//use crate::models::ClearList::*;

pub struct MainHandler<I: IndexClearService> {
    index_clear_service: I,
    clear_index_info: ClusterJson
}

impl<I: IndexClearService> MainHandler<I> {
    
    pub fn new(index_clear_service: I) -> Self {
        
        println!("??");

        //let clear_index_infos: Vec<ClusterIndex> = Vec::new();
        let clear_index_info: ClusterConfig = match read_json_from_file::<ClusterConfig>("./datas/index_list.json") {
            Ok(clear_index_info) => clear_index_info,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e)
            }
        };

        println!("{:?}", clear_index_info);

        Self { index_clear_service, clear_index_info }
    }
    
    
    /*

    */
    pub async fn task_set(&self) -> Result<(), anyhow::Error> {

                

        Ok(())
    }
}