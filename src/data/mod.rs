
use std::path::Path;

use crate::error::VicError;
use crate::map::Map;
use crate::save::Save;

pub struct Info {
    map: Option<Map>,
    saves: Vec<Save>,
    // mods: Vec<Map>?
}


impl Info {
    pub fn new() -> Self {
        Self {
            map: None,
            saves: Vec::new()
        }
    }
    pub fn load_map(&mut self, inp: &Path) -> Result<(), VicError> {
        self.map = Some(Map::new(inp)?);
        Ok(())
    }
    pub fn load_save(&mut self, inp: &Path) -> Result<(), VicError> {
        Ok(self.saves.push(Save::new(inp)?))
    }
}