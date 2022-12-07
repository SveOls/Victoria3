use std::{
    array::IntoIter,
    error, fmt,
    num::{ParseFloatError, ParseIntError},
};

use fltk::prelude::FltkError;
use glob::{GlobError, PatternError};
use zip::result::ZipError;

pub enum VicError {
    MapError(String),
    SaveError,
    Other(Box<dyn error::Error + Send + Sync>),
}

impl VicError {
    pub fn temp() -> Self {
        Self::SaveError
    }
    pub fn named(inp: &str) -> Self {
        Self::MapError(inp.to_owned())
    }
}

impl error::Error for VicError {}

impl fmt::Display for VicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "__")
    }
}

impl fmt::Debug for VicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VicError::MapError(a) => write!(f, "error {}", a),
            VicError::SaveError => write!(f, "aaaaa"),
            VicError::Other(a) => write!(f, "{:?}", a),
        }
    }
}

impl From<Box<dyn error::Error + Send + Sync>> for VicError {
    fn from(error: Box<dyn error::Error + Send + Sync>) -> Self {
        VicError::Other(error)
    }
}

impl From<ParseFloatError> for VicError {
    fn from(error: ParseFloatError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<ParseIntError> for VicError {
    fn from(error: ParseIntError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<FltkError> for VicError {
    fn from(error: FltkError) -> Self {
        VicError::Other(Box::new(error))
    }
}
impl From<GlobError> for VicError {
    fn from(error: GlobError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<std::io::Error> for VicError {
    fn from(error: std::io::Error) -> Self {
        VicError::Other(Box::new(error))
    }
}
impl<const N: usize> From<IntoIter<u8, N>> for VicError {
    fn from(error: IntoIter<u8, N>) -> Self {
        VicError::MapError(format!("{:?}, {N:?}", error.collect::<Vec<_>>()))
    }
}
impl From<PatternError> for VicError {
    fn from(error: PatternError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<ZipError> for VicError {
    fn from(error: ZipError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<std::str::Utf8Error> for VicError {
    fn from(error: std::str::Utf8Error) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<image::ImageError> for VicError {
    fn from(error: image::ImageError) -> Self {
        VicError::Other(Box::new(error))
    }
}

impl From<std::sync::mpsc::RecvError> for VicError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        VicError::Other(Box::new(error))
    }
}
