use crate::error::VicError;
use crate::scanner::{DataFormat, DataStructure, GetMapData, MapIterator};
use std::path::PathBuf;

use crate::wrappers::ColorWrap;

#[derive(Debug)]
pub struct Country {
    name: String,
    color: ColorWrap,
}

impl Country {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn color(&self) -> ColorWrap {
        self.color
    }
}

impl GetMapData for Country {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/country_definitions/*.txt"))
    }
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let mut t_color = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["color", content] => {
                    t_color = Some(ColorWrap::to_colorwrap(
                        MapIterator::new(content, DataFormat::Single).get_val()?,
                    )?)
                }
                _ => {}
            }
        }

        if let Some(color) = t_color {
            Ok(Self { name, color })
        } else {
            unimplemented!()
        }
    }
}
