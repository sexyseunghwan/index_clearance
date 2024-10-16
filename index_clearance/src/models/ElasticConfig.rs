use crate::common::*;
use crate::models::ElasticJson::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElasticConfig {
    pub clusters: Vec<ElasticJson>,
}