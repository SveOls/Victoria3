
use std::error::Error;
use std::io;


pub(crate) trait GetData {
    // type Iter: Iterator<Item = Self>;
    fn consume_one(_: SaveIterator) -> Result<Self, Box<dyn Error>> where Self: Sized;
    // fn consume_vec(_: SaveIterator) -> Result<Self, Box<dyn Error>> where Self: Sized;
}


pub struct SaveIterator<'a>(Box<dyn Iterator<Item = Result<&'a str, Box::<dyn Error>>> + 'a>);



impl<'a> SaveIterator<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut depth = 0;
        let mut para = false;
        let closure = move |c: char| -> bool {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                '"' => para  =! para,
                _ => {}
            }
            c.is_whitespace() && depth == 0 && !para
        };
        SaveIterator(Box::new(data.trim().split(closure).map(|u| u.trim()).filter(|p| !p.is_empty()).map(|y| Ok(y))))
    }
    fn err(error: Box<dyn Error>) -> Self {
        // io::Error::new(io::ErrorKind::Other, format!("test"));
        // SaveIterator(
        //     Box::new("".split_whitespace().map(|x| if true { Err(error) } else { Ok(x) }))
        // )
        unimplemented!("Unimplemented error handling. Panic in Impl Saveiterator.\n{:?}", error);
    }
    fn single(self) -> Option<Result<&'a str, Box<dyn Error>>> {
        unimplemented!()
    }
    pub fn y_str(mut self) -> Result<&'a str, Box<dyn Error>> {
        if let Ok(DataStructure::Val(a)) = self.next().unwrap() {
            Ok(a)
        } else {
            unimplemented!()
        }
    }
    pub fn y_parse<T: std::str::FromStr>(mut self) -> Result<T, Box<dyn Error>> {
        if let Ok(DataStructure::Val(a)) = self.next().unwrap() {
            match a.parse() {
                Ok(a)  => Ok(a),
                Err(_e) => Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("tried passing Val as IntoIterator\n{:?}", a)))),
            }
        } else {
            unimplemented!()
        }
    }
}


pub enum DataStructure<'a> {
    Itr((&'a str, SaveIterator<'a>)),
    Val(&'a str)
}

impl<'a> IntoIterator for DataStructure<'a> {

    type Item       = Result<DataStructure<'a>, Box<dyn Error>>;
    type IntoIter   = SaveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {

        match self {
            DataStructure::Itr((_, ret))    => ret,
            DataStructure::Val(a)           => SaveIterator::err(Box::new(io::Error::new(io::ErrorKind::Other, format!("tried passing Val as IntoIterator\n{:?}", a)))),
        }

    }
}

impl<'a> DataStructure<'a> {
    pub fn name(&self, rhs: &str) -> bool {
        if let DataStructure::Itr((lhs, _)) = self {
            *lhs == rhs
        } else {
            false
        }
    }
    pub fn into_iter_alt(self) -> Result<(&'a str, SaveIterator<'a>), Box<dyn Error>> {
        match self {
            DataStructure::Itr((a, b)) => Ok((a, b)),
            DataStructure::Val(a) => Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("tried passing Val as IntoIteratorV2\n{:?}", a))))
        }
    }
}

impl<'a> Iterator for SaveIterator<'a> {

    type Item = Result<DataStructure<'a>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .and_then(|x| Some(x.and_then(|o| o.split_once(|c| c == '=' || c == '{')
                .map_or_else(
                    ||  Ok(DataStructure::Val(o)),
                    |y| Ok(DataStructure::Itr(
                        (y.0, SaveIterator::new(y.1.strip_prefix('{').and_then(|f| f.strip_suffix('}')).unwrap_or(y.1))))))))
        )
    }
}


