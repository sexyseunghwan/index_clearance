use crate::common::*;

#[derive(Debug, Deserialize)]
pub struct CronConfig {
    pub start_cron: String,
}