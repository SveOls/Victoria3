
use image::Rgb;
use std::error::Error;
use std::path::PathBuf;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

use crate::wrappers::RgbWrap;


#[derive(Debug)]
pub struct NamedColor {
    name:  String,
    color: RgbWrap,
}


impl NamedColor {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn color(&self) -> RgbWrap {
        self.color
    }
}


impl GetMapData for NamedColor {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::get_data_from(inp.join("game/common/named_colors/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, Box<dyn Error>> {

        let [itr_label, content] = inp.itr_info()?;

        let name = itr_label.to_owned();
        let color = RgbWrap::to_rgb(MapIterator::new(content, DataFormat::Single).get_val()?)?;

        Ok(Self {
            name,
            color
        })
    }
    fn consume_vec(inp: MapIterator, _: Option<&str>) -> Result<Vec<Self>, Box<dyn Error>> {
        if let Some(DataStructure::Itr([_, a])) = inp.into_iter().find(|x| x.name("colors")) {
            MapIterator::new(a, DataFormat::Labeled).into_iter().map(|x| Self::consume_one(x)).collect()
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Unimplemented error"))))
        }
    }
}