#[macro_use] extern crate serde_derive;
#[macro_use] extern crate bson;
extern crate mongodb;
extern crate serde_yaml;
extern crate serde_json;

pub mod core;


#[cfg(test)]
mod tests {
    #[test]
    fn test_database() {
        use core::mech::configuration::DatabaseConfiguration;
        use core::mech::databases::mongodb::MongoDatabase;
        use mongodb::Client as MongoClient;
        use mongodb::ThreadedClient;
        use mongodb::db::ThreadedDatabase;
        let handler: String = "mongodb".to_owned();
        let address: String = "127.0.0.1".to_owned();
        let port: i16 = 27017;
        let authenticate: bool = false;
        let username: String = "root".to_owned();
        let password: String = "5up3r53cur3pa55w0rd".to_owned();
        let database: String = "omega".to_owned();
        println!("Creating Database configuration.");
        let db_config: DatabaseConfiguration = DatabaseConfiguration::new(handler, address, port, username, password, authenticate, database);
        println!("Creating MongoDatabase instance.");
        let db_instance: MongoDatabase = MongoDatabase::new(db_config);
        println!("Getting MongoDB connection client.");
        let db_client: MongoClient = db_instance.get_client();
        println!("Testing count function.");
        let db_database = db_client.db("test");
        let db_collection = db_database.collection("test");
        let count_success: bool = match db_collection.count(None, None) {
            Ok(_) => true,
            Err(_) => false
        };
        assert!(count_success);
//        let document = doc! { "test_string": "Alex loves spaghetti." };
//        println!("Testing document insertion.");
//        let insert_success: bool = match db_collection.insert_one(document, None) {
//            Ok(_) => true,
//            Err(_) => false
//        };
//        assert!(insert_success);
    }
}
