use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cert: String,
    pub key: String,
    pub port: Option<u16>,
}
