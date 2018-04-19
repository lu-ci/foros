use core::mech::configuration::DatabaseConfiguration;
use mongodb::Client as MongoClient;
use mongodb::ThreadedClient;
use std::io::Result;
use std::process::exit;

pub struct MongoDatabase {
    config: DatabaseConfiguration,
    client: MongoClient,
}

impl MongoDatabase {
    pub fn new(config: DatabaseConfiguration) -> Self {
        let client = match Self::connect(&config) {
            Ok(client) => client,
            Err(why) => {
                println!("Database Connection Error: {}", why);
                exit(10053);
            }
        };
        Self { config, client }
    }

    fn connect(config: &DatabaseConfiguration) -> Result<MongoClient> {
        let connection_string: String;
        let conn_part: String = format!("{}:{}", config.address, config.port);
        if config.authenticate {
            let auth_part: String = format!("{}:{}", config.username, config.password);
            connection_string = format!("mongodb://{}@{}", conn_part, auth_part);
        } else {
            connection_string = format!("mongodb://{}", conn_part);
        }
        let client = MongoClient::with_uri(&connection_string)?;
        Ok(client)
    }

    pub fn get_config(&self) -> &DatabaseConfiguration {
        &self.config
    }

    pub fn get_client(&self) -> MongoClient {
        self.client.clone()
    }
}
