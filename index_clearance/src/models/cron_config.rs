use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CronConfig {
    pub start_cron: String,
}
