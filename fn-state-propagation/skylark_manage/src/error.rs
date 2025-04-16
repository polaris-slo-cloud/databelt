use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ParseSkylarkKeyError;

impl fmt::Display for ParseSkylarkKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse SkylarkKey")
    }
}

impl error::Error for ParseSkylarkKeyError {}

#[derive(Debug, Clone)]
pub struct SkylarkStateError;

impl fmt::Display for SkylarkStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed fetch SkylarkState")
    }
}

impl error::Error for SkylarkStateError {}