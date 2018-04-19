use std::{
    result::Result as StdResult,
    io,
    fmt::{
        self,
        Display,
        Formatter,
    },
    error::Error as StdError,
};

use serde_json as json;
use mongodb;


#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    JsonError(json::Error),
    MongodbError(mongodb::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::JsonError(ref err) => err.description(),
            Error::MongodbError(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<mongodb::Error> for Error {
    fn from(err: mongodb::Error) -> Self {
        Error::MongodbError(err)
    }
}

pub type Result<T> = StdResult<T, Error>;
