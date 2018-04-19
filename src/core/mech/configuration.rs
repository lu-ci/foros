use serde_json;
use serde_yaml;
use std::{
    fs::File,
    path::Path,
    process::exit,
};

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

    pub fn from_json<P>(location: P) -> Self
        where P: AsRef<Path>
    {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("Discord Config JSON Location Error: {}", why);
                exit(2);
            }
        };

        match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("Discord Config JSON Parsing Error: {}", why);
                exit(8);
            }
        }
    }

    pub fn from_yaml<P>(location: P) -> Self
        where P: AsRef<Path>
    {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("Discord Config YAML Location Error: {}", why);
                exit(2);
            }
        };

        match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("Discord Config YAML Parsing Error: {}", why);
                exit(8);
            }
        }
    }
}

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

    pub fn from_json<P>(location: P) -> Self
        where P: AsRef<Path>
    {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("MongoDB Config JSON Location Error: {}", why);
                exit(2);
            }
        };

        match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("MongoDB Config JSON Parsing Error: {}", why);
                exit(8);
            }
        }
    }

    pub fn from_yaml<P>(location: P) -> Self
        where P: AsRef<Path>
    {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("MongoDB Config YAML Location Error: {}", why);
                exit(2);
            }
        };

        match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("MongoDB Config YAML Parsing Error: {}", why);
                exit(8);
            }
        }
    }
}
