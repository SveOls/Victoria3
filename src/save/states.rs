
use std::error::Error;
use std::io;

use super::pops::Pop;
// use super::super::map::Map;
use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

#[derive(Debug, Default, Clone)]
pub struct State {
    // id
    id:         usize,
    // name
    template_name: String,

    provinces:  Vec<usize>,
    // capital province
    capital:    usize,
    // country id
    country:    usize,
    // pop vec
    pops:       Vec<Pop>
}

impl State {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn state(&self) -> &String {
        &self.template_name
    }
    pub fn provinces(&self) -> impl Iterator<Item = &usize> {
        self.provinces.iter()
    }
    pub fn country(&self) -> usize {
        self.country
    }
    pub fn capital(&self) -> usize {
        self.capital
    }
    pub fn pops(&self) -> impl Iterator<Item = &Pop> {
        self.pops.iter()
    }
    pub fn contains(&self, index: usize) -> bool {
        self.provinces.contains(&index)
    }
    pub fn insert_pop(&mut self, pop: Pop) {
        self.pops.push(pop);
    }
    /// returns (pops of selected culture, total population)
    pub fn culture_pop(&self, culture: usize) -> Result<(usize, usize), Box<dyn Error>> {
        self.pops.iter()
            .map(|p| p.size().and_then(|s| p.culture().and_then(|c| Ok((s, c == culture))))
                .map(|(s, c)| (if c {s} else {0}, s)))
            .try_fold((0, 0), |a, b| b.and_then(|y| Ok((a.0 + y.0, a.1 + y.1))))
    }
    pub fn religion_pop(&self, religion: &str) -> Result<(usize, usize), Box<dyn Error>> {
        let mut ret = (0, 0);
        for pop in &self.pops {
            if pop.religion()? == religion {
                ret.0 += pop.size()?
            }
            ret.1 += pop.size()?
        }
        Ok(ret)
    }
    pub fn pop(&self) -> Result<usize, Box<dyn Error>> {
        self.pops.iter().try_fold(0, |a, b| b.size().map(|x| x + a))
    }
}


impl GetMapData for State {
    fn consume_one(inp: DataStructure) -> Result<Self, Box<dyn Error>> {

        let id;
        let mut t_name      = None;
        let mut t_provinces: Option<Vec<usize>> = None;
        let mut t_capital   = None;
        let mut t_country   = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["capital", content] => {
                    t_capital = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["country", content] => {
                    t_country = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["region", content] => {
                    t_name = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                }
                ["provinces", content] => {
                    for j in MapIterator::new(content, DataFormat::Labeled) {
                        match j.itr_info()? {
                            ["provinces", content_inner] => {
                                let data: Vec<usize> = MapIterator::new(content_inner, DataFormat::MultiVal).get_vec()?.into_iter().map(|x| x.parse()).try_collect()?;
                                let mut ret = Vec::new();
                                for y in 0..data.len()/2 {
                                    for i in data[2*y]..data[2*y] + data[2*y+1] + 1 {
                                        ret.push(i);
                                    }
                                }
                                if let Some(a) = &mut t_provinces {
                                    a.append(&mut ret)
                                } else {
                                    t_provinces = Some(ret)
                                }
                            }
                            _ => panic!()
                        }
                    }
                }
                // if states can be empty (id=none), this is where support for that should be added
                // [_] => unreachable!(),
                _ => {}
            }
        }
        if let (Some(name), Some(provinces), Some(capital), Some(country))
         =     (  t_name,     t_provinces,    t_capital,      t_country) {
            Ok(Self {
                id,
                template_name: name,
                provinces,
                capital,
                country,
                pops: Vec::new()
            })
        } else {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, "Incorrectly Initialized Pop")))
        }

    }
}
