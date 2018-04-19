use std::{
    io::Result,
    process::exit,
};

use mongodb::{
    Client as MongoClient,
    ThreadedClient,
};

use ::configuration::DatabaseConfiguration;


pub struct MongoDatabase {
    pub config: DatabaseConfiguration,
    pub client: MongoClient,
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
}


#[cfg(test)]
mod tests {
    use mongodb::{
        Client as MongoClient,
        ThreadedClient,
        db::ThreadedDatabase,
    };

    use super::{
        DatabaseConfiguration,
        MongoDatabase,
    };


    fn get_db_client() -> MongoClient {
        let db_config: DatabaseConfiguration = DatabaseConfiguration::new(
            "mongodb", "127.0.0.1", 27017, "root", "5up3r53cur3pa55w0rd", false, "omega"
        );
        let db_instance: MongoDatabase = MongoDatabase::new(db_config);
        let db_client: MongoClient = db_instance.client;
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
