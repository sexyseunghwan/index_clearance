use crate::common::*;

use crate::models::ClusterIndexJson::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterIndexConfig {
    pub clusters: Vec<ClusterIndexJson>,
}