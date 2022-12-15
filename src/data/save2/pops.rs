use std::io;

use crate::{
    error::VicError,
    scanner::{DataFormat, DataStructure, GetMapData, MapIterator},
};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Pop {
    id: usize,
    profession: String,
    religion: String,
    culture: usize,
    location: usize,
    workplace: Option<usize>,
    literates: usize,
    workforce: usize,
    dependents: usize,
    wealth: usize,
    empty: bool,
}

impl Pop {
    pub fn location(&self) -> Result<usize, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing location of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(self.location)
        }
    }
    pub fn culture(&self) -> Result<usize, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing culture of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(self.culture)
        }
    }
    pub fn religion(&self) -> Result<&String, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing religion of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(&self.religion)
        }
    }
    pub fn profession(&self) -> Result<&String, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing profession of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(&self.profession)
        }
    }
    pub fn workforce(&self) -> Result<usize, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing size of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(self.workforce)
        }
    }
    pub fn dependents(&self) -> Result<usize, VicError> {
        if self.empty {
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Tried accessing size of empty pop:\n{:?}\n", self),
            ))))
        } else {
            Ok(self.dependents)
        }
    }
    pub fn empty(&self) -> bool {
        self.empty
    }
    pub fn size(&self) -> Result<usize, VicError> {
        Ok(self.dependents()? + self.workforce()?)
    }
}

impl GetMapData for Pop {
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let mut empty: bool = false;
        let id: usize;
        let mut t_profession: Option<String> = None;
        let mut t_religion: Option<String> = None;
        let mut t_culture: Option<usize> = None;
        let mut t_location: Option<usize> = None;
        let mut workplace: Option<usize> = None;
        let mut literates: usize = 0;
        let mut workforce: usize = 0;
        let mut dependents: usize = 0;
        let mut t_wealth: Option<usize> = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["type", content] => {
                    t_profession = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                    if let Some(a) = &mut t_profession {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["size_wa", content] => {
                    workforce = MapIterator::new(content, DataFormat::Single)
                        .get_val()?
                        .parse()?;
                }
                ["size_dn", content] => {
                    dependents = MapIterator::new(content, DataFormat::Single)
                        .get_val()?
                        .parse()?;
                }
                ["location", content] => {
                    t_location = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["literate", content] => {
                    literates = MapIterator::new(content, DataFormat::Single)
                        .get_val()?
                        .parse()?;
                }
                ["religion", content] => {
                    t_religion = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                    if let Some(a) = &mut t_religion {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["wealth", content] => {
                    t_wealth = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["culture", content] => {
                    t_culture = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["workplace", content] => {
                    workplace = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["none"] => empty = true,
                [_] => unreachable!(),
                _ => {}
            }
        }

        if let (Some(profession), Some(religion), Some(culture), Some(location), Some(wealth)) = (
            t_profession.clone(),
            t_religion,
            t_culture,
            t_location,
            t_wealth,
        ) {
            Ok(Self {
                id,
                profession,
                religion,
                culture,
                location,
                workplace,
                literates,
                workforce,
                dependents,
                wealth,
                empty,
            })
        } else if empty {
            let mut ret = Pop::default();
            ret.empty = true;
            ret.id = id;
            Ok(ret)
        } else {
            // print!("{:?}", t_profession);
            // // print!("{:?}", t_religion);
            // print!("{:?}", t_culture);
            // print!("{:?}", t_location);
            // println!("{:?}", workforce);
            // println!("{:?}", dependents);
            // println!("{:?}", t_wealth);
            // println!("{:?}", id);
            Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Incorrectly Initialized Pop",
            ))))
        }
    }
}
