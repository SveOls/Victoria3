

use std::path::PathBuf;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

use crate::wrappers::RgbWrap;
use crate::error::VicError;

#[derive(Debug)]
pub struct Culture {
    name:       String,
    traits:     Vec<String>,
    color:      RgbWrap,
    religion:   String,
    obsessions: Vec<String>,
}


impl Culture {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn color(&self) -> RgbWrap {
        self.color
    }
}


impl GetMapData for Culture {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/cultures/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, VicError> {

        let mut t_traits = None;
        let mut t_color = None;
        let mut t_religion = None;
        let mut obsessions = Vec::new();


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
                ["religion", content] => {
                    t_religion = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned())
                }
                ["obsessions", content] => {
                    obsessions = MapIterator::new(content, DataFormat::MultiVal)
                        .get_vec()?.into_iter()
                        .map(|x| x.to_owned()).collect::<Vec<String>>();
                }
                _ => {}
            }
        }


        if let (Some(traits), Some(color),  Some(religion))
         =     (t_traits,       t_color,    t_religion) {
            // unimplemented!()
            Ok(Self {
                name,
                traits,
                color,
                obsessions,
                religion
            })
        } else {
            unimplemented!()
        }
    }
}