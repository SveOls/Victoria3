use std::io;

use super::pops::Pop;
use crate::error::VicError;
// use super::super::map::Map;
use crate::scanner::{DataFormat, DataStructure, GetMapData, MapIterator};

#[derive(Debug, Default, Clone)]
pub struct State {
    // id
    id: usize,
    // name
    template_name: String,

    provinces: Vec<usize>,
    // capital province
    capital: Option<usize>,
    // country id
    country: Option<usize>,
    // pop vec
    pops: Vec<Pop>,
    empty: bool,
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
    pub fn country(&self) -> Option<usize> {
        self.country
    }
    pub fn capital(&self) -> Option<usize> {
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
    pub fn culture_pop(&self, culture: usize) -> Result<(usize, usize), VicError> {
        self.pops
            .iter()
            .map(|p| {
                p.size()
                    .and_then(|s| p.culture().and_then(|c| Ok((s, c == culture))))
                    .map(|(s, c)| (if c { s } else { 0 }, s))
            })
            .try_fold((0, 0), |a, b| b.and_then(|y| Ok((a.0 + y.0, a.1 + y.1))))
    }
    pub fn religion_pop(&self, religion: &str) -> Result<(usize, usize), VicError> {
        let mut ret = (0, 0);
        for pop in &self.pops {
            if pop.religion()? == religion {
                ret.0 += pop.size()?
            }
            ret.1 += pop.size()?
        }
        Ok(ret)
    }
    pub fn pop(&self) -> Result<usize, VicError> {
        self.pops.iter().try_fold(0, |a, b| b.size().map(|x| x + a))
    }
}

impl GetMapData for State {
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let id;
        let mut t_name = None;
        let mut provinces: Vec<usize> = Vec::new();
        let mut capital = None;
        let mut country = None;
        let mut empty = false;

        let [itr_label, content_outer] = inp.itr_info()?;

        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["capital", content] => {
                    capital = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["country", content] => {
                    country = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["region", content] => {
                    t_name = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                    if let Some(a) = &mut t_name {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["provinces", content] => {
                    for j in MapIterator::new(content, DataFormat::Labeled) {
                        match j.itr_info()? {
                            ["provinces", content_inner] => {
                                let data: Vec<usize> =
                                    MapIterator::new(content_inner, DataFormat::MultiVal)
                                        .get_vec()?
                                        .into_iter()
                                        .map(|x| x.parse())
                                        .try_collect()?;
                                let mut ret = Vec::new();
                                for y in 0..data.len() / 2 {
                                    for i in data[2 * y]..data[2 * y] + data[2 * y + 1] + 1 {
                                        ret.push(i);
                                    }
                                }
                                provinces.append(&mut ret);
                            }
                            _ => panic!(),
                        }
                    }
                }
                ["none"] => empty = true,
                [_] => panic!(),
                _ => {}
            }
        }
        // println!("{:?}", id);
        // println!("{:?}", t_name);
        // println!("{:?}", capital);
        // println!("{:?}", provinces.len());
        // println!("{:?}", country);
        if let Some(name) = t_name {
            Ok(Self {
                id,
                template_name: name,
                provinces,
                capital,
                country,
                pops: Vec::new(),
                empty,
            })
        } else if empty {
            Ok(Self {
                id,
                template_name: String::new(),
                provinces: Vec::new(),
                capital: None,
                country: None,
                pops: Vec::new(),
                empty,
            })
        } else {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Incorrectly Initialized State",
            ))))
        }
    }
}
