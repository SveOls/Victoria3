use std::io;
use std::path::PathBuf;

use glob::glob;
use jomini::{TextToken, TextTape};

use crate::error::VicError;

pub trait GetMapData: Sized {
    fn get_data_from(inp: PathBuf) -> Result<Vec<Self>, VicError> {
        let mut ret = Vec::new();

        for entry in glob(&inp.as_path().to_str().unwrap())? {
            let t_file = std::fs::read(entry?)?;

            let jomda = JomData::new(&t_file);

            let bigiter = JomIter::new(&jomda);

            ret.append(&mut Self::consume_vec(bigiter)?)
        }
        Ok(ret)
    }
    fn consume_vec(mut inp: JomIter) -> Result<Vec<Self>, VicError> {
        let mut ret = Vec::new();
        while let Some(_) = inp.next() {
            ret.push(Self::consume_one(inp.new_array())?)
        }
        Ok(ret)
    }
    fn new_vec(_: PathBuf) -> Result<Vec<Self>, VicError> {
        unimplemented!()
    }
    fn consume_one(_: JomIter) -> Result<Self, VicError> {
        unimplemented!()
    }
}


pub struct JomData<'a> {
    tape: TextTape<'a>,
}
impl<'a> JomData<'a> {
    pub fn new(inp: &'a[u8]) -> Self {
        Self {
            tape: TextTape::from_slice(inp).unwrap()
        }
    }
    fn tokens(&'a self) -> &'a[TextToken<'a>] {
        self.tape.tokens()
    }
}

#[derive(Debug)]
pub struct JomIter<'a> {
    iter: &'a[TextToken<'a>],
    index: usize,
    bounds: Option<[usize; 2]>,
}

impl<'a> JomIter<'a> {
    pub fn new(inp: &'a JomData) -> Self {
        Self {
            iter: inp.tokens(),
            index: 0,
            bounds: None
        }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn _test_rev(&mut self) {
        self.index -= 1
    }
    pub fn new_bounded(&self, bounds: [usize; 2]) -> Self {
        Self {
            iter: self.iter,
            index: self.index,
            bounds: Some(bounds)
        }
    }
    pub fn new_array(&mut self) -> Self {
        let index = self.index + 1;
        let end = match self.next().unwrap() {
            TextToken::Array { end: e, mixed: _ } | TextToken::Object { end: e, mixed: _ } => {
                *e
            }
            TextToken::Quoted(_) | TextToken::Unquoted(_) => {
                self.index
            }
            TextToken::Header(_) => {
                let ret = self.new_array();
                return ret
            }
            a => {
                panic!("{a:?}");
            }
        };
        let ret = Self {
            iter: self.iter,
            index,
            bounds: Some([index, end])
        };
        self.index = end + 1;
        ret
    }
    fn grow_left(&mut self, us: usize) {
        self.bounds = self.bounds.map(|x| [x[0]-us, x[1]]);
        self.index -= 2;
    }
    fn grow_right(&mut self, us: usize) {
        self.bounds = self.bounds.map(|x| [x[0], x[1]+us])
    }
    pub fn name(&self) -> Option<String> {
        self.iter.get(self.index - 2).map(|x| x.as_scalar().map(|x| x.to_string())).flatten()
    }
    pub fn id(&self) -> Option<usize> {
        self.iter.get(self.index - 2).map(|x| x.as_scalar().map(|x| x.to_i64().ok().map(|x| x  as usize))).flatten().flatten()
    }
    pub fn to_value(self) -> Option<String> {
        self.iter.get(self.index).map(|x| x.as_scalar().map(|x| x.to_string())).flatten()
    }
}

impl<'a> Iterator for JomIter<'a> {
    type Item = &'a TextToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bounds.map(|x| x[1] <= self.index).unwrap_or(false) {
            return None
        }
        if let Some(a) = self.iter.get(self.index) {
            match a {
                TextToken::Array { end: e, mixed: _ } | TextToken::Object { end: e, mixed: _ } => {
                    self.index = *e;
                }
                _ => {}
            }
            self.index += 1;
            Some(a)
        } else {
            self.index += 1;
            None
        }
    }
}
