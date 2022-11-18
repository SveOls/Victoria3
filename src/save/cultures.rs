

use regex::Regex;
use std::error::Error;


#[derive(Debug, Default)]
pub struct Culture {
    id:         usize,
    name:       String,
    // could find ID instead of string, but this way the save and game files are analyzed independently.
    homelands:  Vec<String>,
}


impl Culture {
    pub fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Self>, Box<dyn Error>> {


        let mut t_id        = None;
        let mut t_name      = None;
        let mut t_homelands = None;

        let id_reg = Regex::new(r#"^([0-9]+)=\{|type=([a-z_]+)|core_states=\{([A-Z_a-z\s]+)\}|^\}"#).unwrap();


        // println!("entering: {}", a);
        while let Some(a) = data.next() {
            for b in id_reg.captures_iter(&a) {
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return if let (Some(id), Some(name), Some(homelands)) = (t_id, t_name, t_homelands) {
                        Ok(Some(Self { id, name, homelands }))
                    } else {
                        Ok(None)
                    }
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    t_id = Some(c.parse()?);
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    t_name = Some(c.to_owned());
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str())) {
                    t_homelands = Some(c.split(' ').map(|x| x.to_owned()).filter(|x| !x.is_empty()).collect());
                }
            }
        }
        unreachable!()
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn homelands(&self) -> &Vec<String> {
        &self.homelands
    }
}
