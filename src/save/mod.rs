

mod pops;
mod states;
mod countries;
mod cultures;

use pops::Pop;
use states::State;
use countries::Country;
use cultures::Culture;
use super::analyse;
use super::map::Map;

use std::error::Error;
use std::collections::HashMap;

#[derive(Default)]
pub struct Save {
    //meta
    //---
    states:     HashMap<usize, State>,
    countries:  HashMap<usize, Country>,
    cultures:   HashMap<usize, Culture>
}

impl Save {
    pub fn new(inp: &str) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        let (mut save, _) = analyse::analyse(inp)?;

        let mut states = HashMap::new();
        let mut pops = Vec::new();
        let mut countries = HashMap::new();
        let mut cultures = HashMap::new();

        while let Some(line) = save.next() {


            if line == "states={" {
                save.next();
                while let Some(a) = State::new(&mut save)? {
                    states.insert(a.id(), a);
                }
                // states = states::State::new(&mut save)?.into_iter().collect();
            }
            if line == "pops={" {
                save.next();
                // let mut i = 0;
                while let Some(a) = Pop::new(&mut save)? {
                    pops.push(a);
                }
                // pops = pops::Pop::new(&mut save)?.into_iter().collect();

            }
            if line == "country_manager={" {
                save.next();
                // let mut i = 0;
                while let Some(a) = Country::new(&mut save)? {
                    // println!("{:?}", a);
                    countries.insert(a.id()?, a);
                }
                // pops = pops::Pop::new(&mut save)?.into_iter().collect();
            }
            if line == "cultures={" {
                save.next();
                // let mut i = 0;
                while let Some(a) = Culture::new(&mut save)? {
                    // println!("{:?}", a);
                    cultures.insert(a.id(), a);
                }
                // pops = pops::Pop::new(&mut save)?.into_iter().collect();
            }
        }

        for pop in pops.into_iter().filter(|x| !x.empty()) {
            if let Some(state) = states.get_mut(&pop.location()?) {
                state.insert_pop(pop);
            } else {
                panic!("no home for this little guy :(\n\n{:?}", pop);
            }
        }

        ret.states = states;
        ret.countries = countries;
        ret.cultures = cultures;

        Ok(ret)
    }
    pub fn get_tag(&self, index: usize) -> Option<String> {

        for (key, value) in self.states.iter() {
            if value.contains(&index) {
                for countries in self.countries.values() {
                    if let Some(a) = countries.state_id_to_tag(key) {
                        return Some(a)
                    }
                }
            }
        }
        None
    }
    pub fn get_state_id(&self, inp: usize) -> Option<usize> {
        for i in &self.states {
            if i.1.contains(&inp) {
                return Some(*i.0);
            }
        }
        None
    }
    pub fn state_cultures(&self, culture: usize) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
        let mut ret = vec![(0, 0); self.states.len()];
        for (&id, state) in &self.states {
            ret[id] = state.culture_pop(culture)?;
        }
        Ok(ret)
    }
    pub fn country_cultures(&self, culture: usize) -> Result<HashMap<String, (usize, usize)>, Box<dyn Error>> {
        let mut ret = HashMap::new();
        let state_cultures = self.state_cultures(culture)?;
        for (_, country) in &self.countries {
            ret.insert(country.tag().to_owned(), country.states().iter().map(|&x| state_cultures[x]).fold((0, 0), |a, b| (b.0 + a.0, b.1 + a.1)));
        }
        Ok(ret)
    }
    pub fn cultures(&self) -> &HashMap<usize, Culture> {
        &self.cultures
    }
    pub fn area(&self, state: usize, data: &Map) -> Option<usize> {
        if let Some(a) = self.states.get(&state) {
            Some(a.area(data))
        } else {
            None
        }
    }
}