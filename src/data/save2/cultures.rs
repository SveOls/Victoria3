use crate::{
    error::VicError,
    scanner::{DataFormat, DataStructure, GetMapData, MapIterator},
};

#[derive(Debug, Default)]
pub struct Culture {
    id: usize,
    name: String,
    // could find ID instead of string, but this way the save and game files are analyzed independently.
    homelands: Vec<String>,
}

impl Culture {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn homelands(&self) -> impl Iterator<Item = &String> {
        self.homelands.iter()
    }
}

impl GetMapData for Culture {
    fn consume_one(inp: DataStructure) -> Result<Self, VicError> {
        let mut t_name = None;
        let mut homelands = Vec::new();
        let id;

        let [itr_label, content_outer] = inp.itr_info()?;

        id = itr_label.parse()?;

        for i in MapIterator::new(content_outer, DataFormat::Labeled) {
            match i.info() {
                ["type", content] => {
                    t_name = Some(
                        MapIterator::new(content, DataFormat::Single)
                            .get_val()?
                            .to_owned(),
                    );
                }
                ["core_states", content] => {
                    homelands = MapIterator::new(content, DataFormat::MultiVal)
                        .get_vec()?
                        .into_iter()
                        .map(|x| x.to_owned())
                        .collect();
                }
                [_] => unreachable!(),
                _ => {}
            }
        }
        if let Some(name) = t_name {
            Ok(Self {
                id,
                name,
                homelands,
            })
        } else {
            panic!()
        }
    }
}
