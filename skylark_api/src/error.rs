use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct QueryParseError;

impl fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing key and mode from query parameters.")
    }
}

impl error::Error for QueryParseError {}


#[derive(Debug, Clone)]
pub struct SkylarkTopologyError;

impl fmt::Display for SkylarkTopologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error handling topology.")
    }
}

impl error::Error for SkylarkTopologyError {}

#[derive(Debug, Clone)]
pub struct SkylarkPolicyError;

impl fmt::Display for SkylarkPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error computing policy.")
    }
}

impl error::Error for SkylarkPolicyError {}