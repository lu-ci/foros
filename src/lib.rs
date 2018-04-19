#[cfg_attr(test, macro_use)]
extern crate bson;
extern crate mongodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

pub mod configuration;
pub mod databases;
