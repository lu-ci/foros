use std::{
    fs::File,
    path::Path,
};

use serde::de::DeserializeOwned;
use serde_json;
use serde_yaml;

use ::error::Result;


trait FromJson: Sized + DeserializeOwned {
    fn from_json<P>(location: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let file = File::open(location)?;
        let json = serde_json::from_reader(file)?;
        Ok(json)
    }
}

trait FromYaml: Sized + DeserializeOwned {
    fn from_yaml<P>(location: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let file = File::open(location)?;
        let yaml = serde_yaml::from_reader(file)?;
        Ok(yaml)
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

impl FromJson for DiscordConfiguration {}
impl FromYaml for DiscordConfiguration {}


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

impl FromJson for DatabaseConfiguration {}
impl FromYaml for DatabaseConfiguration {}
