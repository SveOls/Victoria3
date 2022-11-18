
use regex::Regex;
use std::error::Error;

use super::pops::Pop;
use super::super::map::Map;

#[derive(Debug, Default)]
pub struct State {
    // id
    id:         usize,
    // name
    name:       String,
    // (start, for). example 1: (10, 5) = 10, 11, 12, 13, 14, 15. Example 2: [(10, 2), (14, 1)] = 10, 11, 12, 14, 15.
    provinces:  Vec<usize>,
    // pop lower, middle, upper
    population: Vec<usize>,
    // capital province
    capital:    usize,
    // country id
    country:    usize,
    // pop vec
    pops:       Vec<Pop>
}

impl State {
    pub fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<State>, Box<dyn Error>> {

        let mut ret = State::default();

        let id_reg = Regex::new(r#"^\tcapital=([0-9]+)|\tregion="([A-Z_]+)"|^\tcountry=([0-9]+)|^\t\tprovinces=\{ ([0-9\s]+) }|^\t\tpop_by_strata=\{ ([0-9\s=]+) }|^\}|^\t\}|^([0-9]+)=\{"#).unwrap();

        let mut capital_set = false;

        while let Some(a) = data.next() {


            // println!("entering: {}", a);
            if let Some(b) = id_reg.captures(&a) {
                // println!("{:?}", b);
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{}", a);
                    if !capital_set {
                        return Ok(None)
                    }
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}\n", ret);
                    if ret.incomplete() {
                        panic!("{:?}", ret);
                    }
                    return Ok(Some(ret))
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    capital_set = true;
                    ret.capital = c;
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str().to_owned())) {
                    ret.name = c;
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    ret.country = c;
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(m.as_str())) {
                    ret.province_collect(c)?;
                }
                if let Some(c) = b.get(5).map_or(None, |m| Some(m.as_str())) {
                    ret.pop_collect(c)?;
                }
                if let Some(c) = b.get(6).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    ret.id = c;
                }
            }
        }
        unreachable!()
    }
    fn pop_collect(&mut self, inp: &str) -> Result<(), Box<dyn Error>> {
        let mut tempest = Vec::new();
        for i in inp.split(' ') {
            let mut temp = i.split('=');
            if let Some(a) = temp.next() {
                if let Some(b) = temp.next() {
                    let id = a.parse::<usize>()?;
                    let val = b.parse::<usize>()?;
                    for _ in tempest.len()..id+1 {
                        tempest.push(0);
                    }
                    tempest[id] = val;
                }
            }
        }
        self.population = tempest;
        Ok(())
    }
    fn province_collect(&mut self, inp: &str) ->  Result<(), Box<dyn Error>> {
        // println!("{}", inp);
        // println!("{:?}", self.name);
        let mut ret = Vec::new();
        let mut data = inp.split(' ').filter_map(|x| x.parse::<usize>().ok());
        while let Some(start_id) = data.next() {
            if let Some(number) = data.next() {
                for i in start_id..start_id + number + 1 {
                    ret.push(i);
                }
            } else {
                panic!()
            }
        }
        self.provinces = ret;
        Ok(())
    }
    pub fn contains(&self, index: &usize) -> bool {
        self.provinces.contains(index)
    }

    /// gotta fix this later. true means initialization of new State failed.
    fn incomplete(&self) -> bool {
        // !self.name.is_some()&self.provinces.is_some()&self.population.is_some()&self.capital.is_some()&self.country.is_some()&self.id.is_some()
        false
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn insert_pop(&mut self, pop: Pop) {
        self.pops.push(pop);
    }
    /// returns (pops of selected culture, total population)
    pub fn culture_pop(&self, culture: usize) -> Result<(usize, usize), Box<dyn Error>> {
        let mut ret = (0, 0);
        for pop in &self.pops {
            if pop.culture()? == culture {
                ret.0 += pop.size()?
            }
            ret.1 += pop.size()?
        }
        Ok(ret)
    }
    pub fn provinces(&self) -> &Vec<usize> {
        &self.provinces
    }
    pub fn area(&self, data: &Map) -> usize {
            data.area(self.provinces())
    }
}
