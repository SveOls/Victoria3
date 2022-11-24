
use image::Rgb;
use std::path::PathBuf;
use crate::error::VicError;
use crate::scanner::{MapIterator, DataFormat, GetMapData, DataStructure};

use crate::wrappers::RgbWrap;

#[derive(Debug)]
pub struct Water {
    lakes: Vec<Rgb<u8>>,
    sea:   Vec<Rgb<u8>>
}

impl Water {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn has(&self, inp: Rgb<u8>) -> bool {
        self.lakes.contains(&inp) || self.sea.contains(&inp)
    }
}

impl GetMapData for Water {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/map_data/*.map"))
    }
    fn consume_one(_:   DataStructure) -> Result<Self, VicError> {
        unreachable!()
    }
    fn consume_vec(inp:   MapIterator, _: Option<&str>) -> Result<Vec<Self>, VicError> {

        let mut t_lakes = None;
        let mut t_sea = None;


        for i in inp {
            match i.itr_info()? {
                ["sea_starts", content] => {
                    t_sea = Some(
                        MapIterator::new(content, DataFormat::MultiVal)
                            .get_vec()?.into_iter()//.inspect(|x| println!("{x}"))
                            .map(|s| RgbWrap::to_rgb(s).map(|x| x.unravel())).try_collect()?
                    );
                }
                ["lakes", content] => {
                    t_lakes = Some(
                        MapIterator::new(content, DataFormat::MultiVal)
                            .get_vec()?.into_iter()//.inspect(|x| println!("{x}"))
                            .map(|s| RgbWrap::to_rgb(s).map(|x| x.unravel())).try_collect()?
                    );
                }
                _ => {}
            }
        }

        // println!("{t_lakes:?}\n\n{t_sea:?}\n\n");

        if let (Some(lakes), Some(sea))
        =     (t_lakes,     t_sea) {
            Ok(vec![Self {
                lakes,
                sea
            }])
        } else {
            unimplemented!()
        }
    }
}