use crate::error::VicError;
use crate::scanner_copy::{GetMapData, JomIter};
use std::path::PathBuf;

use crate::wrappers::ColorWrap;

#[derive(Debug)]
pub struct Country {
    name: String,
    color: ColorWrap,
}

impl Country {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn color(&self) -> ColorWrap {
        self.color
    }
}

impl GetMapData for Country {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/country_definitions/*.txt"))
    }
    fn consume_one(mut inp: JomIter) -> Result<Self, VicError> {
        let mut t_color = None;

        let name = inp.name().unwrap();

        while let Some(i) = inp.next() {
            match &i.as_scalar().map(|x| x.to_string()).as_deref() {
                Some("color") => {
                    let newarr = inp.new_array();
                    let name = newarr.name();
                    let b = newarr.map(|x| x.as_scalar().map(|x| {println!("{}", x); x}).map(|x| x.to_string()).unwrap());
                    let c;
                    if let Some("hsv" | "rgb" | "hsv360") = name.as_deref() {
                        c = std::iter::once(name.unwrap()).chain(b).collect();
                    } else {
                        c = b.collect();
                    }
                    t_color = Some(ColorWrap::from_vec(c)?);
                }
                _ => {}
            }
        }

        if let Some(color) = t_color {
            Ok(Self { name, color })
        } else {
            unimplemented!()
        }
    }
}
