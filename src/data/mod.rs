use std::collections::HashMap;
use std::path::Path;

use crate::error::VicError;
use crate::map::Map;
use crate::save::Save;
use crate::wrappers::RgbWrap;

pub struct Info {
    map: Option<Map>,
    saves: Vec<Save>,
    cur_save: Option<usize>,
}

impl Info {
    pub fn new() -> Self {
        Self {
            map: None,
            saves: Vec::new(),
            cur_save: None,
        }
    }
    pub fn load_map(&mut self, inp: &Path) -> Result<(), VicError> {
        Ok(self.map = Some(Map::new(inp)?))
    }
    pub fn load_save(&mut self, inp: &Path) -> Result<(), VicError> {
        Ok(self.saves.push(Save::new(inp)?))
    }
    pub fn test(&mut self) -> Result<(), VicError> {
        self.cur_save = Some(0);
        let (col, mapper) = self.religion("jewish")?;
        println!("jewish");

        let statenames = self.get_save().unwrap().states().map(|x| (x.id(), x.state().to_owned())).collect::<HashMap<usize, String>>();

        for (key, culturepop) in self.population()? {
            if mapper.get(&key).unwrap() == &0 {
                continue;
            }
            println!("{:?}\t{}\t{}", statenames.get(&key), mapper.get(&key).unwrap(), culturepop)
        }

        Ok(())
    }
    fn get_save(&self) -> Result<&Save, VicError> {
        self.cur_save
            .and_then(|s| self.saves.get(s))
            .ok_or(VicError::SaveError)
    }
    fn get_map(&self) -> Result<&Map, VicError> {
        self.map.as_ref().ok_or(VicError::SaveError)
    }
    /// self.saves
    ///
    ///
    ///
    ///
    pub fn culture(&self, id: usize) -> Result<(Option<RgbWrap>, HashMap<usize, usize>), VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.filter_map(|p| {
                        p.culture()
                            .and_then(|w| (w == id).then(|| p.size()).transpose())
                            .transpose()
                    })
                    .sum::<Result<usize, VicError>>()
                    .map(|k| (s.id(), k))
                })
                .collect::<Result<HashMap<usize, usize>, VicError>>()
                .and_then(|x| {
                    q.get_culture(id)
                        .map(|c| {
                            self.get_map().map(|m| {
                                m.cultures()
                                    .find_map(|l| (l.name() == c.name()).then(|| l.color()))
                            })
                        })
                        .and_then(|o| o.map(|o1| (o1, x)))
                })
        })
    }
    pub fn religion(
        &self,
        religion: &str,
    ) -> Result<(Option<RgbWrap>, HashMap<usize, usize>), VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.filter_map(|p| {
                        p.religion()
                            .and_then(|w| (w == religion).then(|| p.size()).transpose())
                            .transpose()
                    })
                    .sum::<Result<usize, VicError>>()
                    .map(|k| (s.id(), k))
                })
                .collect::<Result<HashMap<usize, usize>, VicError>>()
                .and_then(|x| {
                    self.get_map()
                        .map(|m| {
                            m.religions()
                                .find_map(|l| (l.name() == religion).then(|| l.color()))
                        })
                        .map(|o| (o, x))
                })
        })
    }
    pub fn population(&self) -> Result<HashMap<usize, usize>, VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.map(|p| p.size())
                        .sum::<Result<usize, VicError>>()
                        .map(|k| (s.id(), k))
                })
                .collect::<Result<HashMap<_, _>, VicError>>()
        })
    }
    pub fn area(&self) -> Result<HashMap<usize, usize>, VicError> {
        self.get_save().and_then(|q| {
            q.states()
                .map(|s| {
                    self.get_map()
                        .map(|x| (s.id(), x.area(s.provinces())))
                })
                .collect::<Result<HashMap<_, _>, _>>()
        })
    }
}

//
// Ok(2).and_then(6) = Ok(6)
// Ok(2).map(Ok(6))  = Ok(6)
//
// fn foo() -> Result
// Ok(2).map(|x| foo(x)) => Err(original), Err(from foo) Ok(from foo)
