use std::{
    fs::File,
    path::Path,
};

use serde::de::DeserializeOwned;
use serde_json;
use serde_yaml;

use ::error::{Error, Result};


trait TryFromPath: DeserializeOwned {
    fn try_from_path<P>(location: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let path = location.as_ref();
        let file = File::open(path)?;
        match path.extension() {
            Some(x) if x.to_str() == Some("json") => Ok(serde_json::from_reader(file)?),
            Some(x) if x.to_str() == Some("yml") => Ok(serde_yaml::from_reader(file)?),
            _ => Err(Error::UnknownFiletype),
        }
    }
}


#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    pub bot: bool,
    pub token: String,
    pub owner: i64
}

impl DiscordConfiguration {
    pub fn new<S>(bot: bool, token: S, owner: i64) -> Self
        where S: Into<String>
    {
        Self {
            token: token.into(),
            bot,
            owner
        }
    }
}

impl TryFromPath for DiscordConfiguration {}


#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub handler: String,
    pub address: String,
    pub port: i16,
    pub username: String,
    pub password: String,
    pub authenticate: bool,
    pub database: String
}

impl DatabaseConfiguration {
    pub fn new<S>(handler: S, address: S, port: i16, username: S, password: S, authenticate: bool, database: S) -> Self
        where S: Into<String>
    {
        Self {
            handler: handler.into(),
            address: address.into(),
            username: username.into(),
            password: password.into(),
            database: database.into(),
            authenticate,
            port,
        }
    }
}

impl TryFromPath for DatabaseConfiguration {}
