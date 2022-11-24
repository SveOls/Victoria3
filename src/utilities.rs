
use std::{fs, io};

use crate::error::VicError;

pub fn save(spath: &str, fname: &str, thing: crate::wrappers::ImageWrap) -> Result<(), VicError> {
    let mut buf = std::path::PathBuf::new();
    buf.push(spath);
    match fs::create_dir_all(buf.as_path()) {
        Err(e) if {&e.kind() != &io::ErrorKind::AlreadyExists} =>  return Err(VicError::Other(Box::new(e))),
        _ => {}
    }
    buf.push(fname);
    match fs::remove_file(buf.as_path()) {
        Err(e) if {&e.kind() != &io::ErrorKind::NotFound} => return Err(VicError::Other(Box::new(e))),
        _ => {}
    }
    thing.save(buf.as_path())?;

    Ok(())
}