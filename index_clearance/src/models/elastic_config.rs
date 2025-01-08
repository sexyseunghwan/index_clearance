use crate::common::*;
use crate::models::elastic_info::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElasticConfig {
    pub clusters: Vec<ElasticInfo>,
}
