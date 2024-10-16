use crate::common::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexConfig {
    pub index_pattern: String,
    pub preserve_term: u32,
}