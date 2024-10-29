use crate::common::*;



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


#[derive(Serialize, Deserialize, Debug)]
pub struct SmtpRepository {
    pub smtp_name: String,
    pub credential_id: String,
    pub credential_pw: String,
}