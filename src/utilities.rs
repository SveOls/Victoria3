
use std::{fs, io, error::Error};

pub fn save(spath: &str, fname: &str, thing: crate::wrappers::ImageWrap) -> Result<(), Box<dyn Error>> {
    let mut buf = std::path::PathBuf::new();
    buf.push(spath);
    match fs::create_dir_all(buf.as_path()) {
        Err(e) if {&e.kind() != &io::ErrorKind::AlreadyExists} =>  return Err(Box::new(e)),
        _ => {}
    }
    buf.push(fname);
    match fs::remove_file(buf.as_path()) {
        Err(e) if {&e.kind() != &io::ErrorKind::NotFound} => return Err(Box::new(e)),
        _ => {}
    }
    thing.save(buf.as_path())?;

    Ok(())
}