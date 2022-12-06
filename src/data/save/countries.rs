use crate::{
    error::VicError,
    scanner::{DataFormat, DataStructure, GetMapData, MapIterator},
};

use super::State;

#[derive(Debug, Default)]
pub struct Country {
    not_empty: bool,
    id: usize,
    tag: String,
    capital: usize,
    cultures: Vec<usize>,
    religion: String,
    states: Vec<usize>,
    c_type: String,
}

impl Country {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn tag(&self) -> &String {
        &self.tag
    }
    pub fn states(&self) -> impl Iterator<Item = &usize> {
        self.states.iter()
    }
    pub fn religion(&self) -> &String {
        &self.religion
    }
    pub fn cultures(&self) -> impl Iterator<Item = &usize> {
        self.cultures.iter()
    }
    pub fn c_type(&self) -> &String {
        &self.c_type
    }
    pub fn empty(&self) -> bool {
        !self.not_empty
    }
    pub fn contains(&self, s: &State) -> bool {
        self.states().any(|&sid| sid == s.id())
    }
    // true = fields are missing, false = everything's present
    // fn incomplete(&self) -> bool {
    //     !
    //     self.id         .is_some()&
    //     self.tag        .is_some()&
    //     self.capital    .is_some()&
    //     self.cultures   .is_some()&
    //     self.religion   .is_some()&
    //     self.population .is_some()&
    //     self.states     .is_some()&
    //     self.c_type     .is_some()
    // }
}

impl GetMapData for Country {
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        // println!("ehtyer");
        let id: usize;
        let mut not_empty: bool = true;
        let mut t_tag: Option<String> = None;
        let mut t_capital: Option<usize> = None;
        let mut t_cultures: Option<Vec<usize>> = None;
        let mut t_religion: Option<String> = None;
        let mut states: Vec<usize> = Vec::new();
        let mut t_c_type: Option<String> = None;

        let [itr_label, content_outer] = inp.itr_info()?;
        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["definition", content] => {
                    t_tag = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                    if let Some(a) = &mut t_tag {
                        a.pop();
                        a.remove(0);
                    }
                }
                ["religion", content] => {
                    t_religion = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                }
                ["country_type", content] => {
                    t_c_type = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                }
                ["capital", content] => {
                    t_capital = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .parse()?,
                    );
                }
                ["cultures", content] => {
                    t_cultures = Some(
                        MapIterator::new(content, DataFormat::MultiVal)
                            .get_vec()?
                            .into_iter()
                            .map(|x| x.parse())
                            .collect::<Result<Vec<usize>, std::num::ParseIntError>>()?,
                    );
                }
                ["states", content] => {
                    states = MapIterator::new(content, DataFormat::MultiVal)
                        .get_vec()?
                        .into_iter()
                        .map(|x| x.parse())
                        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()?;
                }
                ["none"] => {
                    not_empty = false;
                }
                [_] => unreachable!(),
                _ => {}
            }
        }

        // println!("{t_tag:?}");
        // println!("{t_capital:?}");
        // println!("{t_cultures:?}");
        // println!("{t_religion:?}");
        // println!("{states:?}");
        // println!("{t_c_type:?}");
        // println!("{id:?}");
        // println!("{not_empty:?}");
        if let (Some(tag), Some(capital), Some(cultures), Some(religion), Some(c_type)) =
            (t_tag, t_capital, t_cultures, t_religion, t_c_type)
        {
            Ok(Self {
                tag,
                capital,
                cultures,
                religion,
                states,
                c_type,
                id,
                not_empty,
            })
        } else if !not_empty {
            let mut ret = Self::default();
            ret.id = id;
            ret.not_empty = false;
            Ok(ret)
        } else {
            Err(VicError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Incorrectly Initialized Country",
            ))))
        }
    }
}
