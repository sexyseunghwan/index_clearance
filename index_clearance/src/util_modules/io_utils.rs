use core::{error, panic};
use std::str::FromStr;

use crate::common::*;

use crate::models::CronConfig::*;



#[doc = "Json 파일을 읽어서 객체로 변환해주는 함수."]
pub fn read_json_from_file<T: DeserializeOwned>(file_path: &str) -> Result<T, anyhow::Error> {
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let data = from_reader(reader)?;
    
    Ok(data)
}


#[doc = "program_schedule 파일을 읽어서 크론식을 객체화 해주는 함수"]
pub fn load_schedule_config() -> Schedule {

    let cron_config: CronConfig = 
        match read_json_from_file::<CronConfig>("./datas/program_schedule.json") {
            Ok(cron_config) => cron_config,
            Err(e) => {
                error!("[Error][load_cron_config()] {:?}", e);
                panic!("{:?}", e)
            }
        };
    
    let schedule: Schedule = 
        match Schedule::from_str(&cron_config.start_cron) {
            Ok(schedule) => schedule,
            Err(e) => {
                error!("[Error][load_cron_config()] {:?}", e);
                panic!("{:?}", e)
            }
        };

    schedule
}