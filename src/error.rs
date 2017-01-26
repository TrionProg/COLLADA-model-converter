use std;
use std::fmt::Display;
use std::fmt;

use collada;

pub enum Error{
    ColladaError(collada::Error),
    StringParseError(String),
    Other(String),
}

impl Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            Error::ColladaError(ref e) => write!(f, "Collada error:{}", e),
            Error::StringParseError(ref message) => write!(f, "String parse error: {}", message),
            Error::Other(ref message) => write!(f, "{}", message),
        }
    }
}
