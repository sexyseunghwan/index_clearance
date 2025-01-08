pub use std::{fs::File, future::Future, io::BufReader, io::Write, str::FromStr, sync::Arc};

pub use tokio::time::{sleep_until, Duration, Instant};

pub use log::{error, info};

pub use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, Record};

pub use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub use serde_json::{from_reader, Value};

pub use elasticsearch::{
    cat::CatIndicesParts,
    http::response::Response,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    http::Url,
    indices::IndicesDeleteParts,
    Elasticsearch,
};

pub use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub use anyhow::{anyhow, Result};

pub use derive_new::new;
pub use getset::Getters;

pub use once_cell::sync::Lazy as once_lazy;

pub use async_trait::async_trait;

pub use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

pub use regex::Regex;

pub use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Transport,
};

pub use futures::future::join_all;
pub use futures::stream::{StreamExt, TryStreamExt};

pub use chrono_tz::Asia::Seoul;

pub use cron::Schedule;

/* 공통전역변수 선언 영역 */
pub static ELASTIC_SERVER_INFO: &str = "./configs/elastic_server_info.toml"; /* Elasticsearch 설정파일 경로 */
pub static SYSTEM_INFO: &str = "./configs/system_config.toml"; /* System 설정파일 경로 */
pub static ELASTIC_INDEX_LIST: &str = "./configs/elastic_index_list.toml"; /* 삭제 대상이 되는 인덱스정보가 존재하는 파일 경로 */
pub static EMAIL_RECEIVER_INFO: &str = "./configs/email_receiver_info.toml"; /* email 수신 대상 정보가 있는 설정파일 경로 */
