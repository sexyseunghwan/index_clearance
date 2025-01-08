use crate::common::*;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct ClusterIndexInfo {
    pub index_pattern: String,
    pub preserve_term: i32,
}

// 아래처럼 한 이유가 있을까?
// pub struct ClusterIndexInfo {
//     pub cluster_name: String,
//     pub index: Vec<IndexConfig>,
// }
