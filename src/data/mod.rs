use std::collections::HashMap;
use std::path::Path;

use crate::error::VicError;
use crate::map::Map;
use crate::save::Save;
use crate::wrappers::ColorWrap;

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
    pub fn clear_saves(&mut self) {
        self.saves = Vec::new()
    }
    pub fn test(&mut self, path: &Path, cul: Option<String>, rel: Option<String>, map: bool, states: bool, black: bool) -> Result<(), VicError> {
        self.cur_save = Some(0);
        // println!("jewish");
        // println!("{} {} {:?}", self.map.is_some(), self.saves.len(), self.cur_save);
        // let (mapper, col) = self.religion("jewish")?;
        // println!("jewish");

        let statenames = self.get_save().unwrap().states().map(|x| (x.id(), x.state().to_owned())).collect::<HashMap<usize, String>>();

        // for (key, culturepop) in self.population()? {
        //     if mapper.get(&key).unwrap() == &0 {
        //         continue;
        //     }
        //     println!("{:?}\t{}\t{}", statenames.get(&key), mapper.get(&key).unwrap(), culturepop)
        // }
        let mut data2;
        if let Some(a) = cul {
            data2 = self.culture2(&a)?;
        } else if let Some(b) = rel {
            data2 = self.religion(&b)?;
        } else if map {
            let a = super::draw::DrawMap::SaveCountries;
            a.draw(path, &[false, true, true, true], self.get_map()?, None, None, None, Some(self.get_save()?), Some(image::Rgb::from([0, 100, 200])))?;
            return Ok(())
        } else if states {
            let a = super::draw::DrawMap::SaveStates;
            a.draw(path, &[false, true, true, true], self.get_map()?, None, None, None, Some(self.get_save()?), Some(image::Rgb::from([0, 100, 200])))?;
            return Ok(())
        } else {
            return Ok(())
        }
        // let mut data2 = (self.area()?, Some(RgbWrap::to_rgb("FFFFFF")?));
        let data3 = self.population()?;
        let mut data3: (HashMap<usize, f64>, Option<ColorWrap>, bool) = (data2.0.iter_mut().map(|(key, val1)| (*key, data3.get(key).map(|x| *val1 as f64 / *x as f64).unwrap_or_else(|| 0.0))).collect(), data2.1, black);

        for i in data3.0.iter().filter(|(_, x)| **x > 0.0).map(|(k, x)| (statenames.get(k), x)) {
            println!("{:?} {:?}", i.0, i.1)
        }

        let max = data3.0.values().fold(0.0f64, |a, &b| a.max(b));
        let min = data3.0.values().fold(1.0f64, |a, &b| a.min(b));
        println!("{max}");
        println!("{min}");
        data3.0.values_mut().for_each(|x| *x = (*x - min)/(max - min));
        // let min = data3.0.values().fold();

        let a = super::draw::DrawMap::SaveStatesData;

        a.draw(path, &[false, true, true, true], self.get_map()?, Some(data3), None, None, Some(self.get_save()?), Some(image::Rgb::from([0, 100, 200])))?;

        Ok(())
    }
    fn get_save(&self) -> Result<&Save, VicError> {
        self.cur_save
            .and_then(|s| self.saves.get(s))
            .ok_or(VicError::MapError(format!("Info tried accessing save with size {:?} at index {:?}", self.saves.len(), self.cur_save)))
    }
    fn get_map(&self) -> Result<&Map, VicError> {
        self.map.as_ref()
            .ok_or(VicError::MapError(format!("Info tried accessing map when map is none (game data not initialized)")))

    }
    pub fn culture(&self, id: usize) -> Result<(HashMap<usize, usize>, Option<ColorWrap>), VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.filter_map(|p| {
                        p.culture()
                            .and_then(|w| (w == id).then(|| p.size()).transpose())
                            .transpose()
                    })
                    .sum::<Result<_, _>>()
                    .map(|k| (s.id(), k))
                })
                .collect::<Result<_, _>>()
                .and_then(|x| {
                    q.get_culture(id)
                        .map(|c| {
                            self.get_map().map(|m| {
                                m.cultures()
                                    .find_map(|l| (l.name() == c.name()).then(|| l.color()))
                            })
                        })
                        .and_then(|o| o.map(|o1| (x, o1)))
                })
        })
    }
    pub fn culture2(&self, test: &str) -> Result<(HashMap<usize, usize>, Option<ColorWrap>), VicError> {
        self.get_map().and_then(|m|
            self.get_save().and_then(|q| {
                q.cultures().find(|i| i.name() == test).map(|y|
                    q.pops()
                        .map(|(s, p)| {
                            p.filter_map(|p| {
                                p.culture()
                                    .and_then(|w| (w == y.id()).then(|| p.size()).transpose())
                                    .transpose()
                            })
                            .sum::<Result<_, _>>()
                            .map(|k: usize| (s.id(), k))
                        })
                        .collect::<Result<HashMap<usize, usize>, VicError>>()
                    .and_then(|x| {
                        q.get_culture(y.id())
                            .map(|c| (x, m.cultures().find_map(|l| (l.name() == c.name()).then(|| l.color()))))
                    })
                    ).unwrap_or(Ok((HashMap::new(), None)))
            })
        )
    }
    pub fn religion(
        &self,
        religion: &str,
    ) -> Result<(HashMap<usize, usize>, Option<ColorWrap>), VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.filter_map(|p| {
                        p.religion()
                            .and_then(|w| (w == religion).then(|| p.size()).transpose())
                            .transpose()
                    })
                    .sum::<Result<_, _>>()
                    .map(|k| (s.id(), k))
                })
                .collect::<Result<_, _>>()
                .and_then(|x| {
                    self.get_map()
                        .map(|m| {
                            m.religions()
                                .find_map(|l| (l.name() == religion).then(|| l.color()))
                        })
                        .map(|o| (x, o))
                })
        })
    }
    pub fn population(&self) -> Result<HashMap<usize, usize>, VicError> {
        self.get_save().and_then(|q| {
            q.pops()
                .map(|(s, p)| {
                    p.map(|p| p.size())
                        .sum::<Result<_, _>>()
                        .map(|k| (s.id(), k))
                })
                .collect()
        })
    }
    pub fn area(&self) -> Result<HashMap<usize, usize>, VicError> {
        self.get_save().and_then(|q| {
            q.states()
                .map(|s| {
                    self.get_map()
                        .map(|x| (s.id(), x.area(s.provinces())))
                })
                .collect()
        })
    }
}

//
// Ok(2).and_then(6) = Ok(6)
// Ok(2).map(Ok(6))  = Ok(6)
//
// fn foo() -> Result
// Ok(2).map(|x| foo(x)) => Err(original), Err(from foo) Ok(from foo)
