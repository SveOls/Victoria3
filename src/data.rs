
use std::path::Path;
use std::error::Error;


use crate::map::Map;
use crate::save::Save;

pub struct Info {
    map: Map,
    saves: Vec<Save>,
    // mods: Vec<Map>?
}


impl Info {
    pub fn new(inp: &Path) -> Result<Self, Box<dyn Error>> {

        let map = Map::new(inp)?;


        Ok(Self {
            map,
            saves: Vec::new()
        })
    }
}