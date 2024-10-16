use crate::common::*;

use crate::repository::es_repository::*;

use crate::util_modules::io_utils::*;
use crate::util_modules::time_utils::*;

use crate::models::ClusterIndexConfig::*;
use crate::models::ClusterIndexJson::*;

#[async_trait]
pub trait IndexClearService {
    async fn delete_index_from_rule(&self) -> Result<(), anyhow::Error>;
}

#[derive(Clone, Debug)]
pub struct IndexClearServicePub<R: EsRepository> {
    elastic_obj: R,
    clear_index_info: ClusterIndexJson
}

impl<R: EsRepository> IndexClearServicePub<R> {
    
    pub fn new(elastic_obj: R) -> Self {
        
        let cluster_name = elastic_obj.get_cluster_name();
        
        let cluster_config: ClusterIndexConfig = match read_json_from_file::<ClusterIndexConfig>("./datas/index_list.json") {
            Ok(clear_index_info) => clear_index_info,
            Err(e) => {
                error!("{:?}", e);
                panic!("{:?}", e)
            }
        };
        
        let clear_index_info = cluster_config.clusters
            .into_iter()
            .find(|cluster| cluster.cluster_name == cluster_name)
            .expect("No matching cluster found for the given name.");

        Self { elastic_obj, clear_index_info }
    } 
}


#[async_trait]
impl<R: EsRepository + Sync> IndexClearService for IndexClearServicePub<R> {   

    async fn delete_index_from_rule(&self) -> Result<(), anyhow::Error> {
        
        let cur_utc_time = get_current_utc_naivedate();
        let target_index = self.clear_index_info.index();
        
        for index_config in target_index {

            let mut delete_index_list: Vec<String> = Vec::new(); // 삭제할 인덱스 대상 벡터.

            let index_pattern = index_config.index_pattern(); // 인덱스 패턴.
            let preserve_term = index_config.preserve_term; // 보존기한.
            
            
            // 인덱스 패턴에 해당되는 모든 인덱스들을 가져온다.
            let res = self.elastic_obj.get_index_belong_pattern(index_pattern).await?;
            
            if let Some(index_obj) = res.as_array() {
                
                for index in index_obj {
                    
                    let index_name = index["index"].as_str()
                        .ok_or_else(|| anyhow!("[Parsing Error][delete_cluster_index()] index['index'] variable not found."))?;
                    
                    let word_split: Vec<&str> = index_name.split('_').collect();
                    let word_split_len = word_split.len();
                    let date = word_split.get(word_split_len - 1)
                        .ok_or_else(|| anyhow!("[Parsing Error][delete_cluster_index()] word_split.get({}) variable not found.", word_split_len))?;
                    
                    let parsed_date = match NaiveDate::parse_from_str(date, "%Y%m%d") {
                        Ok(parsed_date) => parsed_date,
                        Err(e) => {
                            error!("[Parsing Error][delete_cluster_index()] An error occurred while converting 'parsed_date' data. // date: {:?}, {:?}", date ,e);
                            continue
                        }
                    };
                    
                    let perserve_days_ago = cur_utc_time - chrono::Duration::days(preserve_term as i64);
                    
                    
                    if parsed_date < perserve_days_ago {
                        
                        /*
                            **** [[[[ Warning ]]]] ****
                            Index 이름에 log 라는 단어가 없을시에는 제거 대상에 포함되지 않도록 한다.
                            ***검색인덱스를 지울시에 치명적인 장애발생 가능함.***
                        */
                        if ! index_name.contains("log") {
                            info!("The delete target index name MUST CONTAIN the word 'log'. : {}", index_name);
                            continue;
                        }
                        
                        delete_index_list.push(index_name.to_string());
                    }
                }
            }
            
            // 실제 삭제 알고리즘.
            for delete_index in delete_index_list {
                
                self.elastic_obj.delete_index(&delete_index).await?;
                info!("{} index removed", delete_index);
                  
            } 
        }
        
        Ok(())
    }
}