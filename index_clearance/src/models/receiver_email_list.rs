use crate::common::*;

use crate::models::receiver_email::*;

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct ReceiverEmailList {
    pub receivers: Vec<ReceiverEmail>,
}
