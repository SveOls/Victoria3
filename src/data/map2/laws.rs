use crate::{
    error::VicError,
    scanner::{DataFormat, DataStructure, GetMapData, MapIterator},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Law {
    name: String,
    group: String,
}

#[derive(Debug)]
pub struct LawGroup {
    name: String,
    category: String,
}

impl Law {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn group(&self) -> &String {
        &self.name
    }
}

impl LawGroup {
    pub fn new(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::new_vec(inp)
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn category(&self) -> &String {
        &self.name
    }
}

impl GetMapData for Law {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/laws/*.txt"))
    }
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let mut t_group = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["group", content] => {
                    t_group = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    )
                }
                _ => {}
            }
        }
        // println!("{} {:?}", name, t_group);

        if let Some(group) = t_group {
            Ok(Self { name, group })
        } else {
            unreachable!()
        }
    }
}

impl GetMapData for LawGroup {
    fn new_vec(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        Self::get_data_from(inp.join("game/common/law_groups/*.txt"))
    }
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let mut t_category = None;

        let [itr_label, content_outer] = inp.itr_info()?;

        let name = itr_label.to_owned();

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.itr_info()? {
                ["law_group_category", content] => {
                    t_category = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    )
                }
                _ => {}
            }
        }
        // println!("{} {:?}", name, t_category);

        if let Some(category) = t_category {
            Ok(Self { name, category })
        } else {
            unreachable!()
        }
    }
}
