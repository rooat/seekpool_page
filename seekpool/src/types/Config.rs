use crate::toml;
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Config {
    pub url :Url
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Url {
    pub host: String,
    pub explorer : String,
    pub rpc : String,
    pub website : String
}

impl Config {
    pub fn default() -> Self {
        let config_str = include_str!("../configs/config.toml");
        let conf : Self = toml::from_str(config_str).unwrap();
        conf
    }
}
