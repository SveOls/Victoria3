
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




    Stuff::new(Some("save"), stert);
    panic!();


    printer(stert.get(0..1000).unwrap());


    Ok(())
}

fn printer(inp: &str) {

    // let mut parser = delimited::<_, _, _, _, nom::error::Error<_>, _, _, _>(tag("("), tag("abc"), tag(")"));

    println!("{:?}", parser("{abc}"));
    println!("{:?}", parser("{abc}def"));
    println!("{:?}", parser(""));
    println!("{:?}", parser("22{123234234234234234}11"));
    println!("{:?}", parser("{123234234234234234}11"));
    println!("{:?}", parser(inp));

    // let inp = "ablataetoof{rrwd}gefe";
    // let a = parens(inp).unwrap();
    // println!("{}", a.0);
    // println!();
    // println!("{}", a.1);

}

fn parser(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}

// enum StrType<'a> {
//     Utf8(Stuff<'a>),
//     Asci(Stuff<'a>),
// }

struct Stuff<'a> {
    it:     Box<dyn Iterator<Item = (Option<&'a str>, Stuff<'a>)>>,
    data:   &'a str
}

impl<'a> Stuff<'a> {
    fn new(name: Option<&str>, data: &str) -> Stuff<'a> {
        let mut depth = 0;
        let closure = |c: char| -> bool {
            // std::thread::sleep(std::time::Duration::from_millis(100));
            // print!("{}({depth})", c);
            // if c.is_whitespace() {
            //     println!("{depth}");
            // }
            match c {
                '{' => {
                    depth += 1;
                    false
                }
                '}' => {
                    depth -= 1;
                    false
                }
                c1 if depth == 0 && c1.is_whitespace() => true,
                _ => false
            }
        };
        // let it =
        // data.split(|c: char| c.is_whitespace()).filter(|s| !s.is_empty()).for_each(|x| println!("{}", x));
        for i in data.split(closure).inspect(|x| panic!("{}", x)).filter(|s| !s.is_empty()).enumerate() {

            // let a = i.split('=');
            println!("{:?}\n", i.0);
            println!("{}", i.1);
            wait()
        }
        // let ret = Stuff {
        //     it,
        //     data
        // };
        unimplemented!()
    }
}

fn wait() {
    use std::io::{stdin,stdout,Write};
    println!("Please enter some text: ");
    let mut a = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut a);
}