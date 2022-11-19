


use regex::Regex;
use std::error::Error;

use super::save_scanner::{GetData, SaveIterator, DataStructure};

#[derive(Debug, Default)]
pub struct Country {
    // empty
    empty:      bool,
    // id
    id:         usize,
    // name
    tag:        String,
    // capital province
    capital:    usize,
    // primary cultures
    cultures:   Vec<usize>,
    // state religion
    religion:   String,
    // population
    population: Vec<usize>,
    // states
    states:     Vec<usize>,
    // type
    c_type:     String,
}

impl Country {
    pub fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Country>, Box<dyn Error>> {

        let     t_empty      : Option<bool>       = Some(false);
        let mut t_id         : Option<usize>      = None;
        let mut t_tag        : Option<String>     = None;
        let mut t_capital    : Option<usize>      = None;
        let mut t_cultures   : Option<Vec<usize>> = None;
        let mut t_religion   : Option<String>     = None;
        let mut t_population : Option<Vec<usize>> = None;
        let mut t_states     : Option<Vec<usize>> = None;
        let mut t_c_type     : Option<String>     = None;

        let id_reg = Regex::new(r#"([0-9]+)=none|^\}|^\t\}|^([0-9]+)=\{|^\tdefinition="([A-Z]+)"|^\tcapital=([0-9]+)|^\tcultures=\{ ([0-9\s]+) \}|^\treligion=([A-z]+)|^\t\tlower_strata_pops=([0-9]+)|^\t\tmiddle_strata_pops=([0-9]+)|^\t\tupper_strata_pops=([0-9]+)|^\tstates=\{ ([0-9\s]+) \}|^\tcountry_type="([a-z_]+)""#).unwrap();

        // let mut i = 0;
        while let Some(a) = data.next() {

            // println!("entering: {}", a);
            if let Some(b) = id_reg.captures(&a) {
                // println!("{:?}", b);
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{}", a);
                    if t_id.is_none() {
                        return Ok(None)
                    }
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}\n", ret);
                    return if let ( Some(empty), Some(id), Some(tag), Some(capital), Some(cultures), Some(religion), Some(population), Some(states), Some(c_type))
                    =             ( t_empty,     t_id,     t_tag,     t_capital,     t_cultures,     t_religion,     t_population,     t_states,     t_c_type) {
                        Ok(Some(Self {
                            empty,
                            id,
                            tag,
                            capital,
                            cultures,
                            religion,
                            population,
                            states,
                            c_type
                        }))
                    } else {
                        panic!()
                    }
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    let mut ret = Self::default();
                    ret.id = c.parse().unwrap();
                    ret.empty = true;
                    return Ok(Some(ret))
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    if t_id.is_none() {
                        t_id = Some(c.parse().unwrap());
                    }
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str())) {
                    t_tag = Some(c.to_owned());
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(m.as_str())) {
                    t_capital = Some(c.parse()?);
                }
                if let Some(c) = b.get(5).map_or(None, |m| Some(m.as_str())) {
                    t_cultures = Some(c.split(' ').map(|x| x.parse().unwrap()).collect());
                }
                if let Some(c) = b.get(6).map_or(None, |m| Some(m.as_str())) {
                    t_religion = Some(c.to_owned());
                }
                if let Some(c) = b.get(7).map_or(None, |m| Some(m.as_str())) {
                    Self::insert_pop(&mut t_population, 0, c)?;
                }
                if let Some(c) = b.get(8).map_or(None, |m| Some(m.as_str())) {
                    Self::insert_pop(&mut t_population, 1, c)?;
                }
                if let Some(c) = b.get(9).map_or(None, |m| Some(m.as_str())) {
                    Self::insert_pop(&mut t_population, 2, c)?;
                }
                if let Some(c) = b.get(10).map_or(None, |m| Some(m.as_str())) {
                    t_states = Some(c.split(' ').map(|x| x.parse().unwrap()).collect());
                }
                if let Some(c) = b.get(11).map_or(None, |m| Some(m.as_str())) {
                    t_c_type = Some(c.to_owned());
                }
            }
        }
        unreachable!("Country constructor never exited. Return value currently: NaN")
    } // ^53=\{
    fn insert_pop(mut t_pop: &mut Option<Vec<usize>>, strata: usize, pop: &str) -> Result<(), Box<dyn Error>> {
        if t_pop.is_none() {
            *t_pop = Some(vec![0; 3])
        }
        if let Some(popvec) = &mut t_pop {
            popvec[strata] = pop.parse()?;
            Ok(())
        } else {
            unreachable!("Option should be made into Some in previous line; currently None. At: country::Country::insert_pop")
            // Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test")))
        }
    }
    pub fn id(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self.id)
    }
    pub fn state_id_to_tag(&self, index: &usize) -> Option<String> {
        if self.states.contains(index) {
            Some(self.tag.clone())
        } else {
            None
        }
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn states(&self) -> &Vec<usize> {
        &self.states
    }
    // true = fields are missing, false = everything's present
    // fn incomplete(&self) -> bool {
    //     !
    //     self.id         .is_some()&
    //     self.tag        .is_some()&
    //     self.capital    .is_some()&
    //     self.cultures   .is_some()&
    //     self.religion   .is_some()&
    //     self.population .is_some()&
    //     self.states     .is_some()&
    //     self.c_type     .is_some()
    // }
}


impl GetData for Country {
    fn consume_one(_: SaveIterator) -> Result<Self, Box<dyn Error>> {
        Ok(Self::default())
    }
}