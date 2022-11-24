

// use itertools::Itertools;

use image::Rgb;

use std::path::Path;
use std::collections::HashMap;

mod strategic;
mod countries;
mod professions;
mod water;
mod traits;
mod laws;
mod named_colors;
pub mod cultures;
pub mod religions;
pub mod statetemplates;

// use statetemplates::StateTemplate;
use crate::error::VicError;
use strategic::StrategicRegion;
use statetemplates::StateTemplate;
use countries::Country;
use water::Water;
use traits::Trait;
use named_colors::NamedColor;
use religions::Religion;
use professions::Profession;
use cultures::Culture;
use laws::{Law, LawGroup};

use crate::wrappers::RgbWrap;


// #[derive(Debug)]
// pub enum ScanReligion<'a> {
//     Refs(&'a Religion),
//     Name(String)
// }

// #[derive(Debug)]
// pub enum ScanCulture<'a> {
//     Refs(&'a Culture<'a>),
//     Name(String)
// }

// #[derive(Debug)]
// pub enum ScanStates<'a> {
//     Refs(&'a StateTemplate),
//     Name(String)
// }



#[derive(Debug, Default)]
pub struct Map {
    regions:        Vec<StrategicRegion>,
    states:         Vec<StateTemplate>,
    countries:      Vec<Country>,
    religions:      Vec<Religion>,
    cultures:       Vec<Culture>,
    professions:    Vec<Profession>,
    water:          Vec<Water>,
    named_colors:   Vec<NamedColor>,
    laws:           Vec<Law>,
    lawgroups:      Vec<LawGroup>,
    traits:         Vec<Trait>,
    index_color:    Vec<(Rgb<u8>, usize)>,
    provinces:      HashMap<Rgb<u8>, usize>,
}


impl Map {
    pub fn new(inp: &Path) -> Result<Self, VicError> {
        println!("scan start");

        use std::thread;
        use std::sync::mpsc;

        let(tx1, rx1) = mpsc::channel();
        let(tx2, rx2) = mpsc::channel();
        let(tx3, rx3) = mpsc::channel();

        let inp2 = inp.to_path_buf();
        thread::spawn(move || {
            tx1.send((
                Water::             new(inp2.clone()),
                StrategicRegion::   new(inp2.clone()),
                StateTemplate::     new(inp2.clone()),
            ))
        });
        let inp3 = inp.to_path_buf();
        thread::spawn(move || {
            tx2.send((
                Country::           new(inp3.clone()),
                NamedColor::        new(inp3.clone()),
                Culture::           new(inp3.clone()),
                Religion::          new(inp3.clone()),
            ))
        });
        let inp4 = inp.to_path_buf();
        thread::spawn(move || {
            tx3.send((
                LawGroup::        new(inp4.clone()),
                Law::             new(inp4.clone()),
                Profession::      new(inp4.clone()),
                Trait::           new(inp4.clone()),
            ))
        });
        let tempest1 = rx1.recv()?;
        let mut regions     = tempest1.1?;
        let mut states      = tempest1.2?;
        let water           = tempest1.0?;

        for (id, state) in states.iter_mut().enumerate() {
            state.set_id(id);
        }
        for (id, region) in regions.iter_mut().enumerate() {
            region.set_id(id);
        }

        'outer: for state in &mut states {
            if let Some(provov) = state.provinces() {
                for province in provov {
                    for i in &water {
                        if i.has(*province) {
                            state.set_ocean(true);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        let mut offset = 1;
        for region in &mut regions {
            region.set_offset(offset);
            for state in &mut states {
                if region.states().contains(state.name()) {
                    state.set_offset(offset);
                    offset += state.size();
                }
            }
        }

        let img = crate::wrappers::ImageWrap::new(inp.to_path_buf(), None)?;

        let mut index_color: Vec<(Option<Rgb<u8>>, usize)> = vec![(None, 0); states.iter().map(|x| x.size()).sum::<usize>() + 1];

        let mut provinces = HashMap::new();

        let mut unorganized = Vec::new();

        for &img_pixel in img.vflip_pixels() {

            if let Some(&val) = provinces.get(&img_pixel) {
                index_color[val as usize].1 += 1;
            } else {
                let mut change = false;
                'outer: for state in &states {
                    if state.contains(img_pixel) {
                        if let Some(pixel_offset) = state.get_offset() {

                            change = true;

                            for i in pixel_offset..pixel_offset+state.size() {
                                if index_color[i].0.is_none() {

                                    index_color[i].0 = Some(img_pixel);
                                    index_color[i].1 += 1;
                                    provinces.insert(img_pixel, i);

                                    break 'outer;
                                }
                            }
                            unreachable!();
                        }
                    }
                }
                if !change {
                    index_color.push((Some(img_pixel), 1));
                    provinces.insert(img_pixel, index_color.len() - 1);
                    unorganized.push(img_pixel);
                }
            }
        }
        index_color[0] = (Some(Rgb::from([0; 3])), 0);

        let tempest2 = rx2.recv()?;
        let tempest3 = rx3.recv()?;
        let countries       = tempest2.0?;
        let cultures        = tempest2.2?;
        let religions       = tempest2.3?;
        let named_colors    = tempest2.1?;
        let lawgroups       = tempest3.0?;
        let laws            = tempest3.1?;
        let professions     = tempest3.2?;
        let traits          = tempest3.3?;

        println!("scan end");
        Ok(Self {
            cultures,
            countries,
            religions,
            regions,
            traits,
            lawgroups,
            named_colors,
            laws,
            professions,
            states,
            provinces,
            water,
            index_color: index_color.iter().map(|&x| (x.0.unwrap(), x.1)).map(|x| (Rgb::from(x.0), x.1)).collect()
        })
    }

    pub fn get_strat_color(&self, prov: Rgb<u8>) -> Option<Rgb<u8>> {
        for state in &self.states {
            if state.contains(prov) {
                return self.regions.iter()
                    .filter(|x| x.states().iter().find(|&z| z == state.name()).is_some())
                    .find_map(|y| y.color().map(|f| f.unravel()));
            }
        }
        None
        // Err(Box::new(io::Error::new(io::ErrorKind::Other, "couldn't find strat color")))
    }
    /// Returns state name if province belongs to known state template.
    ///
    /// Returns "UNORGANIZED" if province is known (located on province map), but was not found in a state template.
    ///
    /// Returns None if it wasn't found in provinces.png nor in any state template.
    pub fn get_state_name(&self, color: Rgb<u8>) -> Option<&String> {
        self.states.iter().find(|x| x.contains(color)).map(|x| x.name())
    }
    ///
    pub fn to_index(&self, inp: Rgb<u8>) -> Option<usize> {
        self.provinces.get(&inp).cloned()
    }
    pub fn get_country_color(&self, tag: &str) -> Option<Rgb<u8>> {
        self.countries.iter().find(|&x| x.name() == tag).map(|y| y.color().unravel())
    }
    pub fn area(&self, provs: &Vec<usize>) -> usize {
        provs.iter().map(|&x| self.index_color[x].1).sum()
    }
    pub fn get_region(&self, id: usize) -> Option<&StrategicRegion> {
        self.regions.get(id)
    }
    pub fn regions(&self) -> &Vec<StrategicRegion> {
        &self.regions
    }
    pub fn col_index_to_state(&self, id_1: usize) -> Option<&StateTemplate> {
        let color = self.index_color[id_1].0;
        self.states.iter().find(|state| state.contains(color))
    }
    pub fn state_area(&self) -> impl Iterator<Item = (&StateTemplate, usize)> {//-> impl Iterator<Item = (StateTemplate, usize)> {
        self.states.iter().map(|x| (x, self.index_color.iter().filter(|y| x.contains(y.0)).fold(0, |a, b| a + b.1)))
    }
    pub fn job_color(&self, name: &str) -> Option<RgbWrap> {
        self.professions.iter().find(|x| x.name() == name).map(|p| p.color())
    }
}