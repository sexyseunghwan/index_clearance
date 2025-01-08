use crate::common::*;

use crate::models::cluster_index_info::*;

#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct ClusterIndexConfig {
    pub cluster_name: String,
    pub index: Vec<ClusterIndexInfo>
}
