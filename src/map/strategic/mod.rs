
use regex::Regex;
use image::Rgb;

use std::error::Error;

pub mod statetemplates;

use statetemplates::StateTemplate;
use super::analyse;

#[derive(Debug, Default)]
pub struct StrategicRegion {
    id:         Option<usize>,
    name:       Option<String>,
    color:      Option<Rgb<u8>>,
    capital:    Option<Rgb<u8>>,
    culture:    Option<String>,
    states:     Vec<StateTemplate>,
    offset:     usize,
    ocean:      bool,
}

impl StrategicRegion {
    ///
    pub fn new(data: &mut impl Iterator<Item = String>, states: &mut Vec<Option<StateTemplate>>, offset: usize, ocean: bool) -> Result<Option<Self>, Box<dyn Error>> {

        let mut ret = Self::default();
        ret.offset = offset;
        ret.ocean = ocean;

        let id_reg = Regex::new(r#"^([a-z_0-9]+) = \{|^\tgraphical_culture = "([a-z_]+)"|^\tcapital_province = x([0-9a-fA-F]+)|^\tmap_color = \{ ([0-9.\s]+) \}|^\tstates = \{ ([A-Za-z_\s0-9]+) \}|^\}"#).unwrap();

        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                // println!("{:?}", b);
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(Some(ret))
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    ret.name = Some(c.to_owned());
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    ret.culture = Some(c.to_owned());
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str())) {
                    ret.capital = Some(Rgb::from([u8::from_str_radix(&c[0..2], 16).unwrap(), u8::from_str_radix(&c[2..4], 16).unwrap(), u8::from_str_radix(&c[4..6], 16).unwrap()]));
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(m.as_str())) {
                    // let mut temp = [255; 3];
                    // let mut temp2 = [0.0; 3];
                    // // println!("{}", c);
                    // for (index, i) in c.split(' ').enumerate() {
                    //     temp2[index] = i.parse::<f64>()?;
                    // }
                    // if temp2.iter().sum::<f64>() > 3.0 {
                    //     temp[0] = temp2[0] as u8;
                    //     temp[1] = temp2[1] as u8;
                    //     temp[2] = temp2[2] as u8;
                    // } else {
                    //     temp[0] = (temp2[0] * 255.0) as u8;
                    //     temp[1] = (temp2[1] * 255.0) as u8;
                    //     temp[2] = (temp2[2] * 255.0) as u8;
                    // }
                    // ret.color = Some(Rgb::from(temp));
                    // println!("{:?}", ret.name);
                    // println!("{:?}", temp);
                    // println!("{:?}", ret.name);
                    ret.color = Some(analyse::to_rgb(c)?);
                }
                if let Some(c) = b.get(5).map_or(None, |m| Some(m.as_str())) {
                    let names: Vec<String> = c.split(' ').map(|x| x.to_owned()).collect();
                    let mut temp = Vec::new();
                    for i in names {
                        let mut inp = None;
                        for a in states.iter_mut() {
                            if let Some(j) = a {
                                if j.name() == i {
                                    if j.is_ocean() ^ ret.ocean {
                                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Mismatch ocean status between strat region {:?} ({}) and state {:?} ({})", ret.name, ret.ocean, j.name(), j.is_ocean()))))
                                    }
                                    std::mem::swap(&mut inp, a);
                                    temp.push(inp.unwrap());
                                    break;
                                }
                            }
                        }
                    }
                    let mut sum = 0;
                    temp.sort_by(|a, b| a.id().0.partial_cmp(&b.id().0).unwrap());
                    for i in &mut temp {
                        i.set_offset(sum);
                        sum += i.size();
                    }
                    ret.states = temp;
                }
            }
        }
        Ok(None)
    }
    ///
    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id)
    }
    ///
    pub fn size(&self) -> usize {
        self.states.iter().map(|x| x.size()).sum()
    }
    ///
    pub fn get_number(&self, color: Rgb<u8>) -> Result<Option<(usize, usize)>, Box<dyn Error>> {
        // self.states.iter().filter_map(|x| self.get_)
        self.states.iter().filter(|state| state.contains(color)).map(|state| Ok((self.offset + state.offset()?, state.size()))).next().transpose()
        // for state in self.states.iter() {
        //     if state.contains(color) {
        //         return Ok(Some((self.offset + state.offset()?, state.size())))
        //     }
        // }
        // Ok(None)
    }
    /// returns color of strategic region IF "color" matches a province in the region.
    ///
    /// if "known" is added, only state (id = known) will be checked.
    ///
    /// Some example format:
    ///
    /// get_color(0xF1F22A, None) => Some(Rgb<u8> { 0xF1F22A })
    ///
    /// None example format:
    ///
    /// get_color(0x2A552A, Some(&StateTemplate { ... })) => None
    pub fn get_color(&self, color: Rgb<u8>, known: Option<&StateTemplate>) -> Option<Rgb<u8>> {
        if let Some(state) = known {
            if state.contains(color) {
                self.color
            } else {
                None
            }
        } else {
            self.states.iter().filter(|state| state.contains(color)).filter_map(|_| self.color).next()
        }
        // for state in self.states.iter() {
        //     if state.contains(color) {
        //         return self.color
        //     }
        // }
        // None
    }
    /// returns ref to state template if it's in this strategic region.
    ///
    /// Some example format:
    ///
    /// get_state_info(0xF1F22A) => Some(&StateTemplate { id: 69, name: STATE_DISTRICT_OF_COLUMBIA, ..... })
    ///
    /// None example format:
    ///
    /// get_state_info(0x78A611) => None
    pub fn get_state(&self, color: Rgb<u8>) -> Option<&StateTemplate> {
        self.states.iter().find(|state| state.contains(color))

        // for state in self.states.iter() {
        //     if state.contains(color) {
        //         return Some(&state)
        //     }
        // }
        // None
    }
    pub fn states(&self) -> &Vec<StateTemplate> {
        &self.states
    }
}

