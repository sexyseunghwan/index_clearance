use crate::common::*;

use crate::models::IndexConfig::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterJson {
    pub cluster_name: String,
    pub index: Vec<IndexConfig>,
}