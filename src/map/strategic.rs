

use image::Rgb;

use std::error::Error;
use std::path::PathBuf;

use crate::wrappers::RgbWrap;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

#[derive(Debug, Default)]
pub struct StrategicRegion {
    id:         Option<usize>,
    name:       String,
    color:      Option<RgbWrap>,
    capital:    Option<Rgb<u8>>,
    culture:    Option<String>,
    states:     Vec<String>,
    offset:     Option<usize>,
    ocean:      bool,
}

impl StrategicRegion {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::new_vec(inp)
    }
    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id)
    }
    pub fn states(&self) -> &Vec<String> {
        &self.states
    }
    pub fn set_offset(&mut self, inp: usize) {
        self.offset = Some(inp)
    }
    pub fn get_offset(&self) -> Option<usize> {
        self.offset
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn color(&self) -> Option<RgbWrap> {
        self.color
    }
}

impl GetMapData for StrategicRegion {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, Box<dyn Error>> {
        Self::get_data_from(inp.join("game/common/strategic_regions/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, Box<dyn Error>> {

        let mut color = None;
        let mut capital = None;
        let mut culture = None;
        let mut t_states = None;


        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();
        for data in MapIterator::new(content_outer, DataFormat::Labeled) {
            match data.itr_info()? {
                ["graphical_culture", content] => {
                    culture = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned())
                }
                ["map_color", content] => {
                    color = Some(RgbWrap::to_rgb(MapIterator::new(content, DataFormat::Single).get_val()?)?)
                }
                ["capital_province", content] => {
                    let c = MapIterator::new(content, DataFormat::Single).get_val()?;
                    capital = Some(RgbWrap::to_rgb(c)?.unravel());
                }
                ["states", content] => {
                    t_states = Some(MapIterator::new(content, DataFormat::MultiVal)
                        .get_vec()?.into_iter()
                        .map(|x| x.to_owned()).collect());
                }
                _ => {}
            }
        }
        // println!("{name:?} {color:?} {capital:?} {culture:?} {t_states:?}\n\n");

        if let Some(states)
         =      t_states {
            // unimplemented!();
            Ok(Self {
                id: None,
                name,
                color,
                capital,
                culture,
                offset: None,
                ocean: false,
                states,
            })
        } else {
            unimplemented!()
        }
    }
}