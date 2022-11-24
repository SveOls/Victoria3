

use std::error::Error;
use std::path::PathBuf;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

use crate::wrappers::RgbWrap;


#[derive(Debug)]
pub struct Religion {
    name:   String,
    traits: Vec<String>,
    color:  RgbWrap,
    taboos: Vec<String>,
}


impl Religion {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}


impl GetMapData for Religion {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::get_data_from(inp.join("game/common/religions/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, Box<dyn Error>> {

        let mut t_traits = None;
        let mut t_color = None;
        let mut taboos = Vec::new();

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["traits", content] => {
                    t_traits = Some(
                        MapIterator::new(content, DataFormat::MultiVal)
                            .get_vec()?.into_iter()
                            .map(|x| x.to_owned()).collect::<Vec<String>>()
                    );
                }
                ["color", content] => {
                    t_color = Some(RgbWrap::to_rgb(MapIterator::new(content, DataFormat::Single).get_val()?)?)
                }
                ["taboos", content] => {
                    taboos = MapIterator::new(content, DataFormat::MultiVal)
                        .get_vec()?.into_iter()
                        .map(|x| x.to_owned()).collect::<Vec<String>>();
                }
                _ => {}
            }
        }
        // println!("{t_traits:?} {t_color:?} {taboos:?}\n\n");

        if let (Some(traits), Some(color))
         =     (t_traits,       t_color) {
            Ok(Self {
                name,
                traits,
                color,
                taboos
            })
        } else {
            unimplemented!()
        }
    }
}