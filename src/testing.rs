// no warnings for testing stuff
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use serde;
use glob::{glob, Paths, PatternError};
use image::{ImageBuffer, Pixel};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;
use std::{fs, io};

use crate::error::VicError;
use jomini::{JominiDeserialize, TextDeserializer};

#[derive(JominiDeserialize, Debug)]
struct MetaData {
    save_game_version: u64,
    version: String,
    game_date: String,
    name: String,
}

#[derive(JominiDeserialize, Debug)]
struct Date {
    date: String
}

#[derive(JominiDeserialize, Debug)]
struct Pops {
    pops: Vec<Pop>
}

#[derive(JominiDeserialize, Debug)]
struct Pop {
    id: usize,
}

#[derive(JominiDeserialize, Debug)]
struct Save {
    meta_data: MetaData,
}

pub fn jomini() -> Result<(), VicError> {
    let mut zipper: zip::ZipArchive<std::fs::File>;
    let mut writer: Vec<u8> = vec![];
    let file: zip::read::ZipFile;
    let f: std::fs::File;

    f = File::open(format!(
        "/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3/save games/{}.v3",
        "great britain_1836_01_01"
    ))?;

    zipper = zip::ZipArchive::new(f)?;
    file = zipper.by_name("gamestate")?;

    let mut file = file;
    io::copy(&mut file, &mut writer)?;
    // let data = std::str::from_utf8(&writer)?;

    let a = jomini::TextTape::from_slice(&writer).unwrap();

    for i in a.tokens().iter().enumerate() {
        // println!("{:?}", i.1);
        if i.0 == 89 {
            break;
        }
    }

    // let actual: Save = TextDeserializer::from_utf8_slice(&writer).unwrap();

    // println!("{:?}", actual);
    // panic!("{}", stert);

    Ok(())
}



pub fn wait() -> bool {
    use std::io::{stdin, stdout, Write};
    println!("keep this? y/n ");
    let mut a = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut a);
    println!(">{a:?}<");
    if a == "y\n" {
        true
    } else if a == "n\n" {
        false
    } else {
        wait()
    }
}
