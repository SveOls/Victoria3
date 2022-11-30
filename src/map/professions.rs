


use std::path::PathBuf;
use crate::error::VicError;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

use crate::wrappers::ColorWrap;


#[derive(Debug)]
pub struct Profession {
    name: String,
    color: ColorWrap,
    strata: String,
}


impl Profession {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn strata(&self) -> &String {
        &self.strata
    }
    pub fn color(&self) -> ColorWrap {
        self.color
    }
}


impl GetMapData for Profession {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/pop_types/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, VicError> {

        let mut t_color = None;
        let mut t_strata = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["color", content] => {
                    t_color = Some(ColorWrap::to_colorwrap(MapIterator::new(content, DataFormat::Single).get_val()?)?)
                }
                ["strata", content] => {
                    t_strata = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned())
                }
                _ => {}
            }
        }

        if let (Some(color), Some(strata))
         =     (t_color, t_strata) {
            Ok(Self {
                name,
                color,
                strata
            })
        } else {
            unimplemented!()
        }
    }
}