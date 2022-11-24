

mod pops;
mod states;
mod countries;
mod cultures;

use pops::Pop;
use states::State;
use countries::Country;
use cultures::Culture;

use crate::error::VicError;
use crate::scanner::{DataStructure, MapIterator, GetMapData, DataFormat};

use std::{io, io::ErrorKind};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Default)]
pub struct Save {
    //meta
    //---
    states:     HashMap<usize, State>,
    countries:  HashMap<usize, Country>,
    cultures:   HashMap<usize, Culture>,
    date:       Vec<usize>,
}

impl Save {

    pub fn new(inp: &Path) -> Result<Self, VicError> {

        Self::new_vec(inp.to_path_buf()).map(|mut x| x.remove(0))

    }
    pub fn get_owners(&self, province: usize) -> Option<(&State, &Country)> {
        self.states.values()
            .find(|s| s.contains(province))
        .and_then(|s| self.countries.values()
            .find(|c| c.contains(s))
        .map(|c| (s, c)))
    }
    pub fn state_cultures(&self, culture: usize) -> Result<Vec<(usize, usize)>, VicError> {
        let mut ret = vec![(0, 0); self.states.len()];
        for (&id, state) in &self.states {
            ret[id] = state.culture_pop(culture)?;
        }
        Ok(ret)
    }
    pub fn country_cultures(&self, culture: usize) -> Result<HashMap<String, (usize, usize)>, VicError> {
        let mut ret = HashMap::new();
        let state_cultures = self.state_cultures(culture)?;
        for (_, country) in &self.countries {
            ret.insert(country.tag().to_owned(), country.states().map(|&x| state_cultures[x]).fold((0, 0), |a, b| (b.0 + a.0, b.1 + a.1)));
        }
        Ok(ret)
    }
    pub fn cultures(&self)  -> impl Iterator<Item = &Culture> {
        self.cultures.values()
    }
    pub fn states(&self)    -> impl Iterator<Item = &State> {
        self.states.values()
    }
    pub fn countries(&self) -> impl Iterator<Item = &Country> {
        self.countries.values()
    }
    pub fn get_culture(&self, index: usize) -> Result<&Culture, VicError> {
        self.cultures.get(&index)
            .ok_or(VicError::Other(Box::new(io::Error::new(ErrorKind::Other, format!("Couldn't find culture with ID {index} in save")))))
    }
    pub fn get_state(&self, index: usize) ->  Result<&State, VicError> {
        self.states.get(&index)
            .ok_or(VicError::Other(Box::new(io::Error::new(ErrorKind::Other, format!("Couldn't find state with ID {index} in save")))))
    }
    pub fn get_country(&self, index: usize) ->  Result<&Country, VicError> {
        self.countries.get(&index)
            .ok_or(VicError::Other(Box::new(io::Error::new(ErrorKind::Other, format!("Couldn't find country with ID {index} in save")))))
    }
}

impl GetMapData for Save {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp)
    }

    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {

        // let terr = || -> io::Error { io::Error::new(io::ErrorKind::Other, format!("default error consume save")) };


        let mut t_pops      : Option<Vec<Pop>> = None;
        let mut t_states    : Option<HashMap<usize, State>> = None;
        let mut t_countries : Option<HashMap<usize, Country>> = None;
        let mut t_cultures  : Option<HashMap<usize, Culture>> = None;
        let mut t_date      : Option<Vec<usize>> = None;


        let content_outer = inp.itr_info()?[1];

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["pops", content] => {
                    // println!("!!! pops");
                    t_pops = Some(Pop::consume_vec(MapIterator::new(content, DataFormat::Labeled), Some("database"))?)
                }
                ["states", content] => {
                    // println!("!!! states");
                    t_states = Some(State::consume_vec(MapIterator::new(content, DataFormat::Labeled), Some("database"))?.into_iter().map(|x| (x.id(), x)).collect())
                }
                ["country_manager", content] => {
                    // println!("!!! country_manager");
                    t_countries = Some(Country::consume_vec(MapIterator::new(content, DataFormat::Labeled), Some("database"))?.into_iter().map(|x| (x.id(), x)).collect())
                }
                ["cultures", content] => {
                    // println!("!!! cultures");
                    t_cultures = Some(Culture::consume_vec(MapIterator::new(content, DataFormat::Labeled), Some("database"))?.into_iter().map(|x| (x.id(), x)).collect())
                }
                ["date", content] => {
                    // println!("!!! date");
                    t_date = Some(MapIterator::new(content, DataFormat::Single).get_val()?.split('.').map(|x| x.parse()).try_collect()?);
                }
                [_a, _b] => {
                    // println!("{_a:?}")
                }
                _a => {
                    // println!("{_a:?}")
                }
            }
        }
        println!("{t_date:?}");

        if let (Some(pops), Some(countries), Some(cultures), Some(mut states), Some(date))
        =      (t_pops,     t_countries,     t_cultures,     t_states, t_date) {
            for pop in pops.into_iter().filter(|x| !x.empty()) {
                if let Some(state) = states.get_mut(&pop.location()?) {
                    state.insert_pop(pop);
                } else {
                    panic!("no home for this little guy :(\n\n{:?}", pop);
                }
            }
            Ok(Self {
                countries,
                cultures,
                states,
                date
            })
        } else {
            unimplemented!()
        }
    }
    fn get_data_from(inp: PathBuf) -> Result<Vec<Self>, VicError> {

        let mut ret = Vec::new();


        let mut writer: Vec<u8> = vec![];


        match zip::ZipArchive::new(std::fs::File::open(&inp)?) {
            Ok(mut zipper) => {
                let mut file = zipper.by_name("gamestate")?;
                std::io::copy(&mut file, &mut writer)?;
            }
            Err(zip::result::ZipError::InvalidArchive(_)) => {
                writer = std::fs::read(&inp)?;
            }
            Err(e) => {
                return Err(VicError::Other(Box::new(e)))
            }
        }


        let mut comment = false;
        let mut para = false;
        let closure = move |&c: &char | -> bool {
            if c == '"' {
                para ^= para
            } else if c == '#' && !para {
                comment = true
            } else if c == '\n' && comment {
                comment = false
            }
            !comment
        };

        let data = &std::str::from_utf8(&writer)?.chars().filter(closure).collect::<String>();


        ret.push(Self::consume_one(DataStructure::new(data))?);
        Ok(ret)
    }
}