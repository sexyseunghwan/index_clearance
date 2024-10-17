use crate::common::*;

use crate::models::IndexConfig::*;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct ClusterIndexJson {
    pub cluster_name: String,
    pub index: Vec<IndexConfig>,
}