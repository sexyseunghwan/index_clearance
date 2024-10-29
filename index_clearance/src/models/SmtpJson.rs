use crate::common::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SmtpJson {
    pub smtp_name: String,
    pub credential_id: String,
    pub credential_pw: String,
}