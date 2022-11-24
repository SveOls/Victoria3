


use std::{error::Error, io};

use crate::scanner::{GetMapData, DataStructure, MapIterator, DataFormat};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Pop {
    id:         usize,
    profession: String,
    religion:   String,
    culture:    usize,
    location:   usize,
    workplace:  Option<usize>,
    literates:  usize,
    workforce:  usize,
    dependents: usize,
    wealth:     usize,
    empty:      bool,
}


impl Pop {
    pub fn location(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing location of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.location)
        }
    }
    pub fn culture(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing culture of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.culture)
        }
    }
    pub fn religion(&self) -> Result<&str, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing culture of empty pop:\n{:?}\n", self))))
        } else {
            Ok(&self.religion)
        }
    }
    pub fn workforce(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing size of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.workforce)
        }
    }
    pub fn dependents(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing size of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.dependents)
        }
    }
    pub fn empty(&self) -> bool {
        self.empty
    }
    pub fn size(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self.dependents()? + self.workforce()?)
    }
}


impl GetMapData for Pop {
    fn consume_one(inp: DataStructure) -> Result<Self, Box<dyn Error>> {

        let mut empty:          bool            = false;
        let     id:             usize;
        let mut t_profession:   Option<String>  = None;
        let mut t_religion:     Option<String>  = None;
        let mut t_culture:      Option<usize>   = None;
        let mut t_location:     Option<usize>   = None;
        let mut workplace:      Option<usize>   = None;
        let mut literates:      usize           = 0;
        let mut t_workforce:    Option<usize>   = None;
        let mut t_dependents:   Option<usize>   = None;
        let mut t_wealth:       Option<usize>   = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["type", content] => {
                    t_profession    = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                    if let Some(a) = &mut t_profession {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["size_wa", content] => {
                    t_workforce     = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["size_dn", content] => {
                    t_dependents    = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["location", content] => {
                    t_location      = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["literate", content] => {
                    literates       =      MapIterator::new(content, DataFormat::Single).get_val()?.parse()?;
                }
                ["religion", content] => {
                    t_religion      = Some(MapIterator::new(content, DataFormat::Single).get_val()?.to_owned());
                    if let Some(a) = &mut t_religion {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["wealth", content] => {
                    t_wealth        = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["culture", content] => {
                    t_culture       = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["workplace", content] => {
                    workplace       = Some(MapIterator::new(content, DataFormat::Single).get_val()?.parse()?);
                }
                ["none"] => {
                    empty           = true
                }
                [_] => unreachable!(),
                _ => {}
            }
        }


        if let (Some(profession),       Some(religion), Some(culture),  Some(location), Some(workforce),    Some(dependents),   Some(wealth))
         =     (t_profession.clone(),   t_religion,     t_culture,      t_location,     t_workforce,        t_dependents,       t_wealth) {
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
                empty
            })
        } else if empty {
            let mut ret = Pop::default();
            ret.empty = true;
            ret.id = id;
            Ok(ret)
        } else {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, "Incorrectly Initialized Pop")))
        }

    }
}
