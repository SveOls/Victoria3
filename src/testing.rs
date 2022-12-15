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
use jomini::{JominiDeserialize, TextDeserializer, TextToken, TextTape, Scalar};






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

    use crate::scanner_copy::{JomData, JomIter};

    // let writer = b"

    // ";

    let testdata = JomData::new(&writer);
    let mut testiter = JomIter::new(&testdata);


    let a = super::data::map2::countries::Country::new(std::path::Path::new("/mnt/c/Steam/steamapps/common/Victoria 3").to_path_buf());

    print_test(testiter, 4);

    Ok(())
}
fn print_test(mut inp: crate::scanner_copy::JomIter, mut width: usize) {
    while let Some(tok) = inp.next() {
        match tok {
            jomini::TextToken::Object{end: e, mixed: _} | jomini::TextToken::Array{end: e, mixed: _} => {
                print!("{:<width$}", " ");
                println!("{{ {}", e);
                // std::thread::sleep(std::time::Duration::from_millis(2000));
            },
            jomini::TextToken::End(a) => {
                print!("{:<width$}", " ");
                println!("}} {}", a);
            },
            a => {
                print!("{:<width$}", " ");
                println!("{:?}", a);
                match a {
                    TextToken::Unquoted(u) => {
                        match u.to_string().as_str() {
                            "meta_data" | "ironman" => print_test(inp.new_array(), width + 2),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}


pub fn wait() -> bool {
    use std::io::{stdin, stdout, Write};
    // println!("keep this? y/n ");
    let mut a = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut a);
    // println!(">{a:?}<");
    if a == "y\n" {
        true
    } else if a == "n\n" {
        false
    } else {
        wait()
    }
}
