use crate::common::*;

use crate::models::IndexConfig::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElasticJson {
    pub cluster_name: String,
    pub hosts: Vec<String>,
    pub es_id: String,
    pub es_pw: String
}