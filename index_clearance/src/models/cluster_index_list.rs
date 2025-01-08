use crate::common::*;

use crate::models::cluster_index_config::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClusterIndexList {
    pub clusters: Vec<ClusterIndexConfig>
}
