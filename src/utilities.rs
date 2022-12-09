use std::{fs, io, path::PathBuf};

use crate::error::VicError;


pub fn save3(
    spath: PathBuf,
    thing: &crate::wrappers::ImageWrap,
) -> Result<(), VicError> {
    let mut temp = spath.clone();
    temp.pop();
    match fs::create_dir_all(temp.as_path()) {
        Err(e) if { &e.kind() != &io::ErrorKind::AlreadyExists } => {
            return Err(VicError::Other(Box::new(e)))
        }
        _ => {}
    }
    match fs::remove_file(spath.as_path()) {
        Err(e) if { &e.kind() != &io::ErrorKind::NotFound } => {
            return Err(VicError::Other(Box::new(e)))
        }
        _ => {}
    }
    thing.save(spath.as_path())?;

    Ok(())
}
