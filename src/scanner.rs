use std::io;
use std::path::PathBuf;

use glob::glob;

use crate::error::VicError;

pub trait GetMapData: Sized {
    fn get_data_from(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        let mut ret = Vec::new();

        for entry in glob(&inp.as_path().to_str().unwrap())? {
            let t_file = std::fs::read(entry?)?;

            let mut comment = false;
            let mut para = false;
            let closure = move |&c: &char| -> bool {
                if c == '"' {
                    para ^= para
                } else if c == '#' && !para {
                    comment = true
                } else if c == '\n' && comment {
                    comment = false
                }
                !comment
            };

            let data = &std::str::from_utf8(&t_file)?
                .chars()
                .filter(closure)
                .collect::<String>();

            let temp = MapIterator::new(data, DataFormat::Labeled);

            ret.append(&mut Self::consume_vec(temp, None)?)
        }
        Ok(ret)
    }
    fn new_vec(_: PathBuf) -> Result<Vec<Self>, VicError> {
        unimplemented!()
    }
    fn consume_vec(inp: MapIterator, database: Option<&str>) -> Result<Vec<Self>, VicError> {
        if let Some(a) = database {
            if let Some(DataStructure::Itr([_, a])) = inp.into_iter().find(|x| x.name(a)) {
                MapIterator::new(a, DataFormat::Labeled)
                    .into_iter()
                    .map(|x| Self::consume_one(x))
                    .collect()
            } else {
                Err(VicError::Other(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unimplemented error"),
                ))))
            }
        } else {
            inp.into_iter().map(|x| Self::consume_one(x)).collect()
        }
    }
    fn consume_one(_: DataStructure) -> Result<Self, VicError> {
        unimplemented!()
    }
}

/// The format of the content of b in DataStructure::Itr((a[^label], b[^data])).
///
/// Examples provided for each enum variant.
///
/// Iterator collected into array for readability.
///
/// panics if a MapIterator is built over None.
#[derive(Copy, Clone, Debug)]
pub enum DataFormat {
    /// input: " info={ fe fi fo fum }\n\t "
    ///
    /// output: \["info={ fe fi fo fum }"\]
    ///
    Single,
    /// input: " info={ fe fi fo fum }\n\t "
    ///
    /// output: \["info={", "fe", "fi", "fo", "fum }"\]
    ///
    MultiItr,
    ///
    /// Identical to MultiItr, but returns Val(a) instead of Itr(("", a))
    ///
    MultiVal,
    /// input: " info={ fe fi fo fum }\n\t "
    ///
    /// output: \["info={ fe fi fo fum }"\]
    ///
    /// input: " info={ fe fi fo fum }\n\t info2 = data"
    ///
    /// output: \["info={ fe fi fo fum }", "info2 = data"\]
    ///
    Labeled,
    /// input: " info={ fe fi fo fum }\n\t "
    ///
    /// output: panics!
    ///
    None,
}

pub struct MapIterator<'a> {
    data: Box<dyn Iterator<Item = &'a str> + 'a>,
    version: DataFormat,
}

impl<'a> MapIterator<'a> {
    pub fn new(data: &'a str, format: DataFormat) -> Self {
        let mut para = false;
        let mut eq = false;
        let mut depth = 0;
        let mut comment = false;
        let mut text_encountered = false;
        let mut hsv360 = 0;
        let mut hsv = 0;
        let mut rgb = 0;

        let closure = move |c: char| -> bool {
            match c {
                'r' if rgb == 0 => rgb += 1,
                'g' if rgb == 1 => rgb += 1,
                'b' if rgb == 2 => rgb += 1,
                a if !a.is_whitespace() && a != '{' && a != '}' => rgb = 0,
                _ => {}
            }
            match c {
                'h' if hsv == 0 => hsv += 1,
                's' if hsv == 1 => hsv += 1,
                'v' if hsv == 2 => hsv += 1,
                a if !a.is_whitespace() && a != '{' && a != '}' => hsv = 0,
                _ => {}
            }
            match c {
                'h' if hsv360 == 0 => hsv360 += 1,
                's' if hsv360 == 1 => hsv360 += 1,
                'v' if hsv360 == 2 => hsv360 += 1,
                '3' if hsv360 == 3 => hsv360 += 1,
                '6' if hsv360 == 4 => hsv360 += 1,
                '0' if hsv360 == 5 => hsv360 += 1,
                a if !a.is_whitespace() && a != '{' && a != '}' => hsv360 = 0,
                _ => {}
            }
            // para detects plain text
            para ^= c == '"';
            // comment detects comments
            comment |= c == '#' && !para;
            comment &= c != '\n';
            // encounters text is true if c is text and its not in a comment
            text_encountered |= !c.is_whitespace() && !comment && eq;

            if !para && !comment {
                match c {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    '=' => eq = true,
                    _ => {}
                }
                if match format {
                    // single never splits
                    DataFormat::Single => false,
                    // multi splits on all whitespace
                    DataFormat::MultiVal | DataFormat::MultiItr => c.is_whitespace() && depth == 0,
                    // labeled splits on whitespace if depth == 0 and text encountered after split
                    DataFormat::Labeled => {
                        eq && text_encountered
                            && (c == '\n' || (c == '\t' && hsv != 3 && hsv360 != 6 && rgb != 3))
                            && depth == 0
                    }
                    DataFormat::None => unreachable!(),
                } {
                    eq = false;
                    text_encountered = false;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        let ret_data = data.trim_start_matches('\u{feff}').trim();
        // println!("{}", ret_data);
        let ret_data = match format {
            DataFormat::Labeled | DataFormat::MultiVal | DataFormat::MultiItr => ret_data
                .strip_prefix('{')
                .and_then(|x| Some(x.strip_suffix('}').unwrap().trim()))
                .unwrap_or(ret_data),
            _ => ret_data,
        };

        Self {
            data: Box::new(ret_data.split(closure).filter(|x| !x.is_empty())),
            version: format,
        }
    }
    pub fn get_vec(self) -> Result<Vec<&'a str>, VicError> {
        self.into_iter().map(|x| x.val_info()).collect()
    }
    pub fn get_val(mut self) -> Result<&'a str, VicError> {
        self.next().unwrap().val_info()
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = DataStructure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.data.next()?;
        // println!(">->{temp:?}\n\n");
        match self.version {
            DataFormat::Labeled => {
                // If a data structure that should be labeled *isn't*, return Val(a) instead of Itr((a.0, a.1))
                match temp.split_once('=') {
                    Some(temp2) => Some(DataStructure::Itr([temp2.0.trim(), temp2.1.trim()])),
                    None => Some(DataStructure::Val(temp)),
                }
            }
            DataFormat::Single => Some(DataStructure::Val(temp)),
            DataFormat::MultiItr => Some(DataStructure::Itr(["", temp])),
            DataFormat::MultiVal => Some(DataStructure::Val(temp)),
            DataFormat::None => {
                panic!(
                    "Tried to iterate MapIterator with DataFormat = None: {:?}",
                    self.data.next()
                )
            }
        }
    }
}

/// Itr -> owns tuple (&str, MapIterator).
/// &str is the label of the data in MapIterator.
///
/// Val -> owns &str.
/// &str is data.
///
/// Iterating over MapIterator produces this enum.
///
/// Example:
///
///     orthodox = {
///        	texture = "gfx/interface/icons/religion_icons/orthodox.dds"
///        	traits = {
///        		christian
///        	}
///        	color = { 0.62 0.64 0.6 }
///     }
///
/// Turns into:
///
///     Itr((orthodox, [
///         Itr((texture, [Val("gfx/interface/icons/religion_icons/orthodox.dds")]))
///         Itr((traits, [
///             Val(christian)
///         ]))
///         Itr((color, [Val(0.62), Val(0.64), Val(0.6)]))
///     ]))
///
/// Alternate presentation:
///
///     MapIter_0 = [
///         ...
///         Itr((orthodox, MapIter_1))
///         ...
///     ]
///     MapIter_1 = [
///         Itr((texture, MapIter_2)),
///         Itr((traits, MapIter_3)),
///         Itr((color, MapIter_4))
///     ]
///     MapIter_2 = [
///         Val("gfx/interface/icons/religion_icons/orthodox.dds")
///     ]
///     MapIter_3 = [
///         Val(christian)
///     ]
///     MapIter_4 = [
///         Val(0.62),
///         Val(0.64),
///         Val(0.6)
///     ]
///
#[derive(Debug)]
pub enum DataStructure<'a> {
    Itr([&'a str; 2]),
    Val(&'a str),
}

impl<'a> DataStructure<'a> {
    pub fn name(&self, rhs: &str) -> bool {
        if let DataStructure::Itr([lhs, _]) = self {
            *lhs == rhs
        } else {
            false
        }
    }
    pub fn info(&self) -> &[&'a str] {
        match self {
            DataStructure::Itr(a) => a,
            DataStructure::Val(a) => std::array::from_ref(a),
        }
    }
    pub fn itr_info(self) -> Result<[&'a str; 2], VicError> {
        match self {
            DataStructure::Itr(a) => Ok(a),
            _ => Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Unimplemented error in map_scanner::itr_info: >{self:?}<"),
            )))),
        }
    }
    pub fn new(inp: &'a str) -> Self {
        DataStructure::Itr(["", inp])
    }
    pub fn val_info(self) -> Result<&'a str, VicError> {
        match self {
            DataStructure::Val(a) => Ok(a),
            DataStructure::Itr([a, _]) => Err(VicError::Other(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Unimplemented error in map_scanner::val_info: >{a:?}<"),
            )))),
        }
    }
}
