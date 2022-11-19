
// no warnings for testing stuff
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::{fs, io};
use std::io::{prelude::*, BufReader};
use image::{ImageBuffer, Pixel};
use std::error::Error;
use glob::{glob, Paths, PatternError};
use nom::{
    IResult,
    sequence::delimited,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    bytes::complete::is_not
};
use std::str;
use crate::save::save_scanner::GetData;
use nom::bytes::complete::tag;

pub fn tester() -> Result<(), Box<dyn Error>> {




    let mut zipper: zip::ZipArchive<std::fs::File>;
    let mut writer: Vec<u8> = vec![];
    let file:       zip::read::ZipFile;
    let f:          std::fs::File;


    f = File::open(format!("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3/save games/{}.v3", "great britain_1836_01_01"))?;

    zipper  = zip::ZipArchive::new(f)?;
    file    = zipper.by_name("gamestate")?;

    let mut file = file;
    io::copy(&mut file, &mut writer)?;
    let stert = std::str::from_utf8(&writer)?;


    let sav = super::save::Save::new(stert)?;

    panic!();

    Ok(())
}


// fn printer(mut temp: SaveIterator, depth: usize) {
//     while let Some(data) = temp.next() {
//         if depth == 0 {
//             // wait();
//         }
//         match data {
//             DataStructure::Itr((a, b)) => {
//                 // println!();
//                 for i in 0..depth {
//                     // print!("\t")
//                 }
//                 // print!("{} -> ", a);
//                 printer(b, depth + 1);
//             }
//             DataStructure::Val(a) => {
//                 // println!(" -> {}", a);
//             }
//         }
//     }
// }


// trait GetData {
//     fn consume(&self, inp: SaveIterator) {}
// }


// struct SaveIterator<'a>(Box<dyn Iterator<Item = &'a str> + 'a>);



// impl<'a> SaveIterator<'a> {
//     fn new(data: &'a str, first: bool) -> Self {

//         if !first {
//             // println!("stuff: {:?}", data);
//         }

//         let mut depth = 0;
//         let mut para = false;
//         let mut closure = move |c: char| -> bool {
//             match c {
//                 '{' => depth += 1,
//                 '}' => depth -= 1,
//                 '"' => para  =! para,
//                 _ => {}
//             }
//             c.is_whitespace() && depth == 0 && !para
//         };
//         SaveIterator(Box::new(data.trim().split(closure).map(|u| u.trim()).filter(|p| !p.is_empty())))
//     }
// }

// enum DataStructure<'a> {
//     Itr((&'a str, SaveIterator<'a>)),
//     Val(&'a str)
// }

// impl<'a> DataStructure<'a> {
//     // fn data_harvest()
// }

// impl<'a> Iterator for SaveIterator<'a> {

//     type Item = DataStructure<'a>;

//     fn next(&mut self) -> Option<Self::Item> {

//         self.0.next()//.inspect(|w| println!("\n{:?}\n", w))
//             .and_then(|x| Some(x.split_once(|c| c == '=' || c == '{')//.inspect(|w| println!("\n{:?}\n", w))
//                 .map_or_else(
//                     || DataStructure::Val(x),
//                     |y| DataStructure::Itr((y.0, SaveIterator::new(y.1.strip_prefix('{').and_then(|f| f.strip_suffix('}')).unwrap_or(y.1), false)))))
//             )
//     }
// }


fn wait() {
    use std::io::{stdin,stdout,Write};
    // println!("Please enter some text: ");
    let mut a = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut a);
}

