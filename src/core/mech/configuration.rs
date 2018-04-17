use serde_json;
use serde_yaml;
use std::fs::File;
use std::process::exit;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    bot: bool,
    token: String,
    owner: i64
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
        let config: Self = match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("Discord Config JSON Parsing Error: {}", why);
                exit(8);
            }
        };
        return config;
    }
    pub fn from_yaml(location: String) -> Self {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("Discord Config YAML Location Error: {}", why);
                exit(2);
            }
        };
        let config: Self = match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("Discord Config YAML Parsing Error: {}", why);
                exit(8);
            }
        };
        return config;
    }
    pub fn is_bot(&self) -> bool {
        (&self.bot).to_owned()
    }
    pub fn get_token(&self) -> String {
        (&self.token).to_owned()
    }
    pub fn get_owner(&self) -> i64 {
        (&self.owner).to_owned()
    }
}
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    handler: String,
    address: String,
    port: i16,
    username: String,
    password: String,
    authenticate: bool,
    database: String
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
        let config: Self = match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("MongoDB Config JSON Parsing Error: {}", why);
                exit(8);
            }
        };
        return config;
    }
    pub fn from_yaml(location: String) -> Self {
        let file: File = match File::open(location) {
            Ok(file) => file,
            Err(why) => {
                println!("MongoDB Config YAML Location Error: {}", why);
                exit(2);
            }
        };
        let config: Self = match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(why) => {
                println!("MongoDB Config YAML Parsing Error: {}", why);
                exit(8);
            }
        };
        return config;
    }
    pub fn get_handler(&self) -> String {
        (&self.handler).to_owned()
    }
    pub fn get_address(&self) -> String {
        (&self.address).to_owned()
    }
    pub fn get_port(&self) -> i16 {
        (&self.port).to_owned()
    }
    pub fn get_username(&self) -> String {
        (&self.username).to_owned()
    }
    pub fn get_password(&self) -> String {
        (&self.password).to_owned()
    }
    pub fn authenticate(&self) -> bool {
        (&self.authenticate).to_owned()
    }
    pub fn get_database(&self) -> String {
        (&self.database).to_owned()
    }
}
