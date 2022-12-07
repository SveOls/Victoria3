use std::collections::HashMap;
use std::path::Path;

pub mod map;
pub mod save;

use crate::draw::MapDrawer;
use crate::error::VicError;
use crate::wrappers::ColorWrap;
use map::Map;
use save::Save;

#[derive(Copy, Clone)]
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
    pub fn clear(&mut self, inp: DataTypes) {
        match inp {
            DataTypes::Map => self.map = None,
            DataTypes::Save => self.save = None,
        }
        self.save = None
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
