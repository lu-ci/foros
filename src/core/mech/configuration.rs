use serde_json;
use serde_yaml;
use std::fs::File;
use std::process::exit;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    pub bot: bool,
    pub token: String,
    pub owner: i64
}

impl DiscordConfiguration {
    pub fn new(bot: bool, token: String, owner: i64) -> Self {
        Self { bot, token, owner }
    }

    pub fn from_json(location: String) -> Self {
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

    pub fn from_yaml(location: String) -> Self {
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
    pub fn new(handler: String, address: String, port: i16, username: String, password: String, authenticate: bool, database: String) -> Self {
        Self { handler, address, port, username, password, authenticate, database }
    }

    pub fn from_json(location: String) -> Self {
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

    pub fn from_yaml(location: String) -> Self {
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
