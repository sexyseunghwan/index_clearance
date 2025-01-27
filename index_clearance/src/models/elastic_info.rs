use crate::common::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElasticInfo {
    pub cluster_name: String,
    pub hosts: Vec<String>,
    pub es_id: Option<String>,
    pub es_pw: Option<String>,
}
