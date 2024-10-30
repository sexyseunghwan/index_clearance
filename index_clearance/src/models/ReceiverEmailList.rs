use crate::common::*;

use crate::models::ReceiverEmail::*;

use crate::util_modules::io_utils::*;

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct ReceiverEmailList {
    pub receivers: Vec<ReceiverEmail>,
}

// impl ReceiverEmailList {
    
//     pub fn new() -> Result<Self, anyhow::Error> {
//         let receiver_email_list: ReceiverEmailList = read_json_from_file::<ReceiverEmailList>("./datas/email_receiver.json")?;
//         Ok(receiver_email_list)
//     }
    
// }
