
use std::{error, fmt};

pub(crate) enum VicError {
    MapError(String),
    SaveError,
    Other(Box<dyn error::Error>)
}

impl error::Error for VicError {}

impl fmt::Display for VicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for VicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Box<dyn error::Error>> for VicError {
    fn from(error: Box<dyn error::Error>) -> Self {
        VicError::Other(error)
    }
}