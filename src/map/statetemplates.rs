
use image::Rgb;

use std::path::PathBuf;

use crate::error::VicError;
use crate::wrappers::RgbWrap;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

#[derive(Debug, Default)]
pub struct StateTemplate {
    name:           String,
    // naiive id first, as read from game files; followed by an assigned ID as per the logic of the game.
    // in game, state ID is assigned iteratively, independently of the ID in the game files.
    id:             Option<(usize, usize)>,
    subsistence_b:  Option<String>,
    arable_r:       Option<Vec<String>>,
    capped_r:       Option<Vec<(String, usize)>>,
    provinces:      Option<Vec<Rgb<u8>>>,
    arable_land:    Option<u32>,
    discoverable:   Option<Vec<(Option<String>, Option<String>, Option<u32>, Option<u32>)>>,
    // resources:      Option<Vec<(String, u16)>>,
    ocean:          bool,
    offset:         Option<usize>
}

impl StateTemplate {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    /// checks if state contains province with ID (color).
    pub fn contains(&self, color: Rgb<u8>) -> bool {
        if let Some(provinces) = &self.provinces {
            for &province in provinces {
                if province == color {
                    return true
                }
            }
        }
        false
    }
    pub fn is_ocean(&self) -> bool {
        self.ocean
    }
    pub fn set_id(&mut self, id: usize) {
        if let Some(a) = &mut self.id {
            self.id = Some((a.0, id))
        } else {
            self.id = Some((0, id))
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn id(&self) -> (usize, usize) {
        self.id.unwrap()
    }
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset)
    }
    pub fn get_offset(&self) -> Option<usize> {
        self.offset
    }
    pub fn provinces(&self) -> Option<&Vec<Rgb<u8>>> {
        self.provinces.as_ref()
    }
    pub fn size(&self) -> usize {
        self.provinces.as_ref().map_or(0, |x| x.len())
    }
    pub fn set_ocean(&mut self, ocean: bool) {
        self.ocean = ocean
    }
    pub fn arable_land(&self) -> Option<u32> {
        self.arable_land
    }
}


impl GetMapData for StateTemplate {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/map_data/state_regions/*.txt"))
    }
    fn consume_one(inp:   DataStructure) -> Result<Self, VicError> {

        let mut t_id = None;
        let mut t_provinces = None;
        let mut arable_land = None;
        let mut arable_r = None;
        let mut capped_r = None;
        let mut discoverable: Option<Vec<(Option<String>, Option<String>, Option<u32>, Option<u32>)>> = None;
        let mut subsistence_b: Option<String> = None;


        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();
        for data in MapIterator::new(content_outer, DataFormat::Labeled) {
            match data.itr_info()? {
                ["id", content] => {
                    t_id = Some((MapIterator::new(content, DataFormat::Single).get_val()?.parse()?, 0))
                }
                ["provinces", content] => {
                    let mut temp = Vec::new();
                    for x in MapIterator::new(content, DataFormat::MultiVal).get_vec()? {
                        temp.push(RgbWrap::to_rgb(x)?.unravel())
                    }
                    t_provinces = Some(temp)
                }
                ["arable_land", content] => {
                    arable_land = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?)
                }
                ["subsistence_building", content] => {
                    subsistence_b = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                    if let Some(a) = &mut subsistence_b {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["arable_resources", content] => {
                    let mut temp = Vec::new();
                    for farms in MapIterator::new(content, DataFormat::MultiVal) {
                        let a = farms.val_info()?;
                        temp.push(a.to_owned())
                    }
                    for i in &mut temp  {
                        i.pop();
                        i.remove(0);
                    }
                    arable_r = Some(temp);
                }
                ["capped_resources", content] => {
                    let mut temp = Vec::new();
                    for farms in MapIterator::new(content, DataFormat::Labeled) {
                        let a = farms.itr_info()?;
                        temp.push((a[0].to_owned(), a[1].parse()?))
                    }
                    capped_r = Some(temp);
                }
                ["resource", content] => {
                    let mut tempinner = (None, None, None, None);
                    for resource in MapIterator::new(content, DataFormat::Labeled) {
                        match resource.itr_info()? {
                            ["type", content] => {
                                tempinner.0 = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                                if let Some(a) = &mut tempinner.0 {
                                    a.pop();
                                    a.remove(0);
                                }
                            }
                            ["depleted_type", content] => {
                                tempinner.1 = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                                if let Some(a) = &mut tempinner.1 {
                                    a.pop();
                                    a.remove(0);
                                }
                            }
                            ["undiscovered_amount", content] => {
                                tempinner.2 = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?)
                            }
                            ["discovered_amount", content] => {
                                tempinner.3 = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?)
                            }
                            _ => panic!()
                        }
                    }
                    if let Some(a) = &mut discoverable {
                        a.push(tempinner);
                    } else {
                        discoverable = Some(vec![tempinner])
                    }
                }
                _ => {}
            }
        }

        if let (Some(id), Some(provinces))
         =      (t_id, t_provinces) {
            // unimplemented!();
            Ok(Self {
                id: Some(id),
                name,
                discoverable,
                provinces: Some(provinces),
                arable_land,
                arable_r,
                capped_r,
                subsistence_b,
                ocean: false,
                offset: None
            })
        } else {
            unimplemented!()
        }
    }
}