

use std::path::PathBuf;
use crate::{scanner::{GetMapData, DataStructure, MapIterator, DataFormat}, error::VicError};



#[derive(Debug)]
pub struct Trait {
    name:       String,
    heritage:   bool,
}


impl Trait {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}


impl GetMapData for Trait {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/discrimination_traits/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, VicError> {

        let mut heritage = false;

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["heritage", content] => {
                    heritage = MapIterator::new(content, DataFormat::Single).get_val()? == "yes"
                }
                _ => {}
            }
        }

        Ok(Self {
            name,
            heritage
        })
    }
}