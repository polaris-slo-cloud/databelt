use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct NoKeyGiven;

impl fmt::Display for NoKeyGiven {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no key param given in query")
    }
}

impl error::Error for NoKeyGiven {}