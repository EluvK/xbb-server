use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_path: Option<String>,
    pub cert: String,
    pub key: String,
    pub port: Option<u16>,
    pub latest_version: String,
}
