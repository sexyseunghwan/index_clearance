use crate::common::*;

use crate::models::ClusterJson::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterConfig {
    pub clusters: Vec<ClusterJson>,
}