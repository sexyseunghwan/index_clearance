use crate::common::*;

use crate::util_modules::io_utils::*;

use crate::models::cron_config::*;
use crate::models::smtp_config::*;

static SERVER_CONFIG: once_lazy<Arc<Config>> =
    once_lazy::new(|| Arc::new(initialize_server_config()));

#[doc = "Function to initialize System configuration information instances"]
pub fn initialize_server_config() -> Config {
    info!("initialize_server_config() START!");
    Config::new()
}

#[derive(Debug)]
pub struct Config {
    pub smtp: Arc<SmtpConfig>,
    pub schedule: Arc<CronConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigNotSafe {
    pub smtp: SmtpConfig,
    pub schedule: CronConfig,
}

#[doc = "SMTP config 정보"]
pub fn get_smtp_config_info() -> Arc<SmtpConfig> {
    let smtp_config = &SERVER_CONFIG.smtp;
    Arc::clone(smtp_config)
}

#[doc = "Schedule config 정보"]
pub fn get_schedule_config_info() -> Arc<CronConfig> {
    let schedule_config = &SERVER_CONFIG.schedule;
    Arc::clone(schedule_config)
}

impl Config {
    pub fn new() -> Self {
        let system_config = match read_toml_from_file::<ConfigNotSafe>(SYSTEM_INFO) {
            Ok(system_config) => system_config,
            Err(e) => {
                error!(
                    "[Error][main()] Failed to retrieve information 'system_config'. : {:?}",
                    e
                );
                panic!(
                    "[Error][main()] Failed to retrieve information 'system_config'. : {:?}",
                    e
                );
            }
        };

        Config {
            smtp: Arc::new(system_config.smtp),
            schedule: Arc::new(system_config.schedule),
        }
    }
}
