


use regex::Regex;
use std::{error::Error, io};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Pop {
    id:         usize,
    profession: String,
    religion:   String,
    culture:    usize,
    location:   usize,
    workplace:  Option<usize>,
    literacy:   usize,
    workforce:  usize,
    dependents: usize,
    wealth:     usize,
    empty:      bool,
}


impl Pop {
    pub fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Self>, Box<dyn Error>> {

        let mut t_id:           Option<usize>   = None;
        let mut t_profession:   Option<String>  = None;
        let mut t_religion:     Option<String>  = None;
        let mut t_culture:      Option<usize>   = None;
        let mut t_location:     Option<usize>   = None;
        let mut workplace:      Option<usize>   = None;
        let mut literacy:       usize           = 0;
        let mut t_workforce:    Option<usize>   = None;
        let mut t_dependents:   Option<usize>   = None;
        let mut t_wealth:       Option<usize>   = None;

        let id_reg = Regex::new(r#"^([0-9]+)=\{|^\ttype="([A-z]+)"|^\tsize_wa=([0-9]+)|^\tsize_dn=([0-9]+)|^\tlocation=([0-9]+)|^\tculture=([0-9]+)|^\tworkplace=([0-9]+)|^\treligion="([A-z]+)"|^\tliterate=([0-9]+)|^\twealth=([0-9]+)|^\t\}|^\}|^([0-9]+)=none"#).unwrap();

        while let Some(a) = data.next() {
            // println!("{}", a);

            // println!("entering: {}", a);
            if let Some(b) = id_reg.captures(&a) {
                // println!("PANG: {:?}", b);
                // println!("{:?}", b);
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{}", a);
                    return Ok(None)
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return if let ( Some(id),   Some(profession),       Some(religion), Some(culture),  Some(location), Some(workforce),    Some(dependents),   Some(wealth))
                    =             ( t_id,       t_profession.clone(),   t_religion,     t_culture,      t_location,     t_workforce,        t_dependents,       t_wealth) {
                        let ret = Self {
                            id,
                            profession,
                            religion,
                            culture,
                            location,
                            workplace,
                            literacy,
                            workforce,
                            dependents,
                            wealth,
                            empty: false
                        };
                        Ok(Some(ret))
                    } else {
                        Err(Box::new(io::Error::new(io::ErrorKind::Other, "Incorrectly Initialized Pop")))
                    }
                }
                if let Some(_) = b.get(11).map_or(None, |m| Some(m.as_str())) {
                    let mut ret = Self::default();
                    ret.empty = true;
                    // println!("{:?}", ret);
                    return Ok(Some(ret))
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_id = Some(c);
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str().to_owned())) {
                    t_profession = Some(c);
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_workforce = Some(c);
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_dependents = Some(c);
                }
                if let Some(c) = b.get(5).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_location = Some(c);
                }
                if let Some(c) = b.get(6).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_culture = Some(c);
                }
                if let Some(c) = b.get(7).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    workplace = Some(c);
                }
                if let Some(c) = b.get(8).map_or(None, |m| Some(m.as_str().to_owned())) {
                    t_religion = Some(c);
                }
                if let Some(c) = b.get(9).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    literacy = c;
                }
                if let Some(c) = b.get(10).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    t_wealth = Some(c);
                }
            }
        }
        unreachable!()
    }
    /// returns error if pop empty
    pub fn location(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing location of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.location)
        }
    }
    /// panics if pop is empty
    pub fn culture(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing culture of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.culture)
        }
    }
    /// panics if pop is empty
    pub fn size(&self) -> Result<usize, Box<dyn Error>> {
        if self.empty {
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Tried accessing size of empty pop:\n{:?}\n", self))))
        } else {
            Ok(self.dependents + self.workforce)
        }
    }
    pub fn empty(&self) -> bool {
        self.empty
    }
}
