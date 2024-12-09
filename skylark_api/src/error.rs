use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct QueryParseError;

impl fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing key and mode from query parameters.")
    }
}

impl error::Error for QueryParseError {}