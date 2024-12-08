use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ParseSkylarkKeyError;

impl fmt::Display for ParseSkylarkKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse SkylarkKey")
    }
}

impl error::Error for ParseSkylarkKeyError {}