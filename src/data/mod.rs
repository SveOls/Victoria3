use std::collections::HashMap;
use std::path::Path;

pub mod map;
pub mod save;

use crate::draw::{Coloring, MapDrawer};
use crate::error::VicError;
use crate::wrappers::ColorWrap;
use map::Map;
use save::Save;

pub enum DataTypes {
    Map,
    Save,
}

pub struct Info {
    map: Option<Map>,
    save: Option<Save>,
    drawer: MapDrawer,
}

impl Info {
    pub fn new() -> Self {
        Self {
            map: None,
            save: None,
            drawer: MapDrawer::default(),
        }
    }
    pub fn load(&mut self, inp: &Path, load_type: DataTypes) -> Result<(), VicError> {
        match load_type {
            DataTypes::Map => Ok(self.map = Some(Map::new(inp)?)),
            DataTypes::Save => Ok(self.save = Some(Save::new(inp)?)),
        }
    }
    pub fn clear_save(&mut self) {
        self.save = None
    }
    pub fn clear_map(&mut self) {
        self.save = None
    }
    pub fn test(
        &mut self,
        path: &Path,
        cul: Option<String>,
        rel: Option<String>,
        map: bool,
        states: bool,
        black: bool,
    ) -> Result<(), VicError> {
        let statenames = self
            .get_save()
            .unwrap()
            .states()
            .map(|x| (x.id(), x.state().to_owned()))
            .collect::<HashMap<usize, String>>();

        let mut data2;
        if let Some(a) = cul {
            data2 = self.culture(&a)?;
        } else if let Some(b) = rel {
            data2 = self.religion(&b)?;
        } else if map {
            let a = super::draw::DrawMap::SaveCountries;
            a.draw(
                path,
                &[false, true, true, true],
                self.get_map()?,
                None,
                None,
                None,
                Some(self.get_save()?),
                Some(image::Rgb::from([0, 100, 200])),
            )?;
            return Ok(());
        } else if states {
            let a = super::draw::DrawMap::SaveStates;
            a.draw(
                path,
                &[false, true, true, true],
                self.get_map()?,
                None,
                None,
                None,
                Some(self.get_save()?),
                Some(image::Rgb::from([0, 100, 200])),
            )?;
            return Ok(());
        } else {
            return Ok(());
        }
        // let mut data2 = (self.area()?, Some(RgbWrap::to_rgb("FFFFFF")?));
        let data3 = self.population()?;
        let mut data3: (HashMap<usize, f64>, Option<ColorWrap>, bool) = (
            data2
                .0
                .iter_mut()
                .map(|(key, val1)| {
                    (
                        *key,
                        data3
                            .get(key)
                            .map(|x| *val1 as f64 / *x as f64)
                            .unwrap_or_else(|| 0.0),
                    )
                })
                .collect(),
            data2.1,
            black,
        );

        for i in data3
            .0
            .iter()
            .filter(|(_, x)| **x > 0.0)
            .map(|(k, x)| (statenames.get(k), x))
        {
            println!("{:?} {:?}", i.0, i.1)
        }

        let max = data3.0.values().fold(0.0f64, |a, &b| a.max(b));
        let min = data3.0.values().fold(1.0f64, |a, &b| a.min(b));
        println!("{max}");
        println!("{min}");
        data3
            .0
            .values_mut()
            .for_each(|x| *x = (*x - min) / (max - min));
        // let min = data3.0.values().fold();
        let mut drawer = crate::draw::MapDrawer::default();
        drawer.set_numerator(Some(data2.0));
        drawer.set_denominator(Some(self.population()?));
        drawer.set_color(data2.1);
        drawer.darkmode(false);
        drawer.set_sea_color(ColorWrap::from(image::Rgb::from([0, 100, 200])));
        drawer.set_color_map(Coloring::SaveStates);
        drawer.set_lines(Coloring::SaveCountries);
        drawer.set_path(std::path::PathBuf::from(
            "/mnt/c/Steam/steamapps/common/Victoria 3",
        ));
        println!("start");
        drawer.draw(&self, std::path::PathBuf::from("output"), true)?;
        println!("end");

        Ok(())
    }
    pub fn get_save(&self) -> Result<&Save, VicError> {
        self.save.as_ref().ok_or(VicError::MapError(format!(
            "Info tried accessing save when save is none (save data not initialized)"
        )))
    }
    pub fn get_map(&self) -> Result<&Map, VicError> {
        self.map.as_ref().ok_or(VicError::MapError(format!(
            "Info tried accessing map when map is none (game data not initialized)"
        )))
    }
    pub fn culture(
        &self,
        test: &str,
    ) -> Result<(HashMap<usize, usize>, Option<ColorWrap>), VicError> {
        self.get_map().and_then(|m| {
            self.get_save().and_then(|q| {
                q.cultures()
                    .find(|i| i.name() == test)
                    .map(|y| {
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
                                q.get_culture(y.id()).map(|c| {
                                    (
                                        x,
                                        m.cultures().find_map(|l| {
                                            (l.name() == c.name()).then(|| l.color())
                                        }),
                                    )
                                })
                            })
                    })
                    .unwrap_or(Ok((HashMap::new(), None)))
            })
        })
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
                .map(|s| self.get_map().map(|x| (s.id(), x.area(s.provinces()))))
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
