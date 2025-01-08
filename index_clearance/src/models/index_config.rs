use crate::common::*;

#[derive(Serialize, Deserialize, Getters, Debug, Clone)]
#[getset(get = "pub")]
pub struct IndexConfig {
    pub index_pattern: String,
    pub preserve_term: u32,
}
