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

#[cfg(test)]
mod tests {
    use core::mech::configuration::DatabaseConfiguration;
    use super::MongoDatabase;
    use super::MongoClient;
    use super::ThreadedClient;
    use mongodb::db::ThreadedDatabase;

    fn get_db_client() -> MongoClient {
        let handler: String = "mongodb".to_owned();
        let address: String = "127.0.0.1".to_owned();
        let port: i16 = 27017;
        let authenticate: bool = false;
        let username: String = "root".to_owned();
        let password: String = "5up3r53cur3pa55w0rd".to_owned();
        let database: String = "omega".to_owned();
        let db_config: DatabaseConfiguration = DatabaseConfiguration::new(handler, address, port, username, password, authenticate, database);
        let db_instance: MongoDatabase = MongoDatabase::new(db_config);
        let db_client: MongoClient = db_instance.get_client();
        return db_client;
    }

    #[test]
    fn test_database_count() {
        let db_client: MongoClient = get_db_client();
        let db_database = db_client.db("test");
        let db_collection = db_database.collection("test");
        let count_success: bool = match db_collection.count(None, None) {
            Ok(_) => true,
            Err(_) => false
        };
        assert!(count_success);
    }

    #[test]
    fn test_database_insert() {
        let db_client: MongoClient = get_db_client();
        let db_database = db_client.db("test");
        let db_collection = db_database.collection("test");
        let document = doc! { "test_string": "Alex loves spaghetti." };
        let insert_success: bool = match db_collection.insert_one(document, None) {
            Ok(_) => true,
            Err(_) => false
        };
        assert!(insert_success);
    }

    #[test]
    fn test_database_delete() {
        let db_client: MongoClient = get_db_client();
        let db_database = db_client.db("test");
        let db_collection = db_database.collection("test");
        let document = doc! { "test_string": "Alex loves spaghetti." };
        let delete_success: bool = match db_collection.delete_one(document, None) {
            Ok(_) => true,
            Err(_) => false
        };
        assert!(delete_success);
    }
}
