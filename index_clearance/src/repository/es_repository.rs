use crate::common::*;
use crate::models::ElasticConfig::*;
use crate::util_modules::io_utils::*;

/* 
    Elasticsearch DB 초기화
*/
pub fn initialize_db_clients(es_info_path: &str) -> Result<Vec<EsRepositoryPub>, anyhow::Error> {

    let mut elastic_conn_vec: Vec<EsRepositoryPub> = Vec::new();
    
    let cluster_config: ElasticConfig = read_json_from_file::<ElasticConfig>(es_info_path)?;
    
    for config in &cluster_config.clusters {
        
        let es_helper = EsRepositoryPub::new(
            &config.cluster_name,
            config.hosts.clone(), 
            &config.es_id, 
            &config.es_pw)?;
        
        elastic_conn_vec.push(es_helper);
    }
    
    Ok(elastic_conn_vec)
}


#[async_trait]
pub trait EsRepository {
    async fn delete_index(&self, index_name: &str) -> Result<(), anyhow::Error>;
    async fn get_index_belong_pattern(&self, index_pattern: &str) -> Result<Value, anyhow::Error>;

    fn get_cluster_name(&self) -> String;
}

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct EsRepositoryPub {
    pub cluster_name: String,
    pub es_clients: Vec<EsClient>,
}


#[derive(Debug, Getters, Clone, new)]
pub(crate) struct EsClient {
    host: String,
    es_conn: Elasticsearch
}


impl EsRepositoryPub {
    
    pub fn new(cluster_name: &str, hosts: Vec<String>, es_id: &str, es_pw: &str) -> Result<Self, anyhow::Error> {

        let mut es_clients: Vec<EsClient> = Vec::new();
        
        for url in hosts {
    
            let parse_url = format!("http://{}:{}@{}", es_id, es_pw, url);
            
            let es_url = Url::parse(&parse_url)?;
            let conn_pool = SingleNodeConnectionPool::new(es_url);
            let transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(5,0))
                .build()?;
            
            let elastic_conn = Elasticsearch::new(transport);
            let es_client = EsClient::new(url, elastic_conn);
            es_clients.push(es_client);
        }

        Ok(EsRepositoryPub{cluster_name: cluster_name.to_string(), es_clients})
    }
    
    
    // Common logic: common node failure handling and node selection
    async fn execute_on_any_node<F, Fut>(&self, operation: F) -> Result<Response, anyhow::Error>
    where
        F: Fn(EsClient) -> Fut + Send + Sync,
        Fut: Future<Output = Result<Response, anyhow::Error>> + Send,
    {
        let mut last_error = None;
    
        // StdRng를 사용하여 Send 트레잇 문제 해결
        let mut rng = StdRng::from_entropy(); // 랜덤 시드로 생성
        
        // 클라이언트 목록을 셔플
        let mut shuffled_clients: Vec<EsClient> = self.es_clients.clone();
        shuffled_clients.shuffle(&mut rng); // StdRng를 사용하여 셔플
        
        // 셔플된 클라이언트들에 대해 순차적으로 operation 수행
        for es_client in shuffled_clients {
            match operation(es_client).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    last_error = Some(err);
                }
            }
        }
        
        // 모든 노드에서 실패했을 경우 에러 반환
        Err(anyhow::anyhow!(
            "All Elasticsearch nodes failed. Last error: {:?}",
            last_error
        ))
    }

}

#[async_trait]
impl EsRepository for EsRepositoryPub {
    

    /*
        특정 인덱스 자체를 삭제해주는 함수.
    */
    async fn delete_index(&self, index_name: &str) -> Result<(), anyhow::Error> {

        let response = self.execute_on_any_node(|es_client| async move {
    
            let response = es_client
                .es_conn
                .indices()
                .delete(IndicesDeleteParts::Index(&[index_name]))
                .send()
                .await?;
            
            Ok(response)
        })
        .await?;

        if response.status_code().is_success() {
            Ok(())
        } else {
            let error_message = format!("[Elasticsearch Error][node_delete_query()] Failed to delete document: Status Code: {}", response.status_code());
            Err(anyhow!(error_message))
        }
    }

    /*
        특정 인덱스 패턴에 속하는 인덱스 전부를 가져와주는 함수.
    */
    async fn get_index_belong_pattern(&self, index_pattern: &str) -> Result<Value, anyhow::Error> {
        
        let response = self.execute_on_any_node(|es_client| async move {
    
            let response = es_client
                .es_conn
                .cat()
                .indices(CatIndicesParts::Index(&[index_pattern]))
                .format("json")
                .send()
                .await?;
            
            Ok(response)
        })
        .await?;
        
        if response.status_code().is_success() {
            let response_body = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            let error_message = format!("[Elasticsearch Error][node_delete_query()] Failed to delete document: Status Code: {}", response.status_code());
            Err(anyhow!(error_message))
        }

        

    }


    /*
        Elasticsearch 클러스터의 이름을 가져와주는 함수.
    */
    fn get_cluster_name(&self) -> String {
        self.cluster_name().to_string()
    }
    
}