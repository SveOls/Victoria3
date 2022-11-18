

// use zip;
// use std::io::Read;

// pub fn test() -> String {

//     let file = std::fs::File::open("data/great britain_1836_01_01.v3").unwrap();

//     let mut archive = zip::ZipArchive::new(file).unwrap();

//     let mut file = archive.by_name("gamestate").unwrap();

//     println!("Filename: {}", file.name());
//     println!("Filename: {}", file.compression());
//     println!("Filename: {:?}", file.unix_mode());
//     println!("Filename: {}", file.name());

//     let mut test = String::new();

//     file.read_to_string(&mut test).unwrap();

//     // println!("{test}");
//     test
// }


use std::fs::File;
use std::{fs, io};
use std::io::{prelude::*, BufReader};
use std::error::Error;
use image::{ImageBuffer, Rgb, Pixel};
use glob::{glob, Paths, PatternError};

///accepts the following formats:
///hsv{ a b c }
///hsv360{ d e f }
///{ a b c }
///{ d e f }
pub fn to_rgb(inp: &str) -> Result<Rgb<u8>, Box<dyn Error>> {
    // println!("{:?}", inp);
    let mut temp = [0u8; 3];
    let mut vals: Vec<f64>;
    if inp.get(0..6) == Some("hsv360") {

        let vals: Vec<usize> = inp.split(|c| c == ' '||c == '{'||c == '}').filter_map(|x| x.parse::<usize>().ok()).collect();

        let c = ((vals[1]*vals[2]) as f64) / 10000.0;
        let m = (vals[2] as f64 / 100.0) - c;

        // temp[vals[0]/60] // 0 1 2 3 4 5 6
        //
        // 1 0 2 1 0 2
        // (1 + 2x)%3
        //
        // 0 1 1 2 2 0
        // (3x)%3
        //
        // 2 2 0 0 1 1
        // ((2 + x)/2)%3
        //
        // println!("{} {}", c, m);
        // println!("{}", ((1 +      (vals[0]/60))/2)%3  );
        // println!("{}", (1 + 2*    (vals[0]/60))%3     );
        // println!("{}", ((4 +      (vals[0]/60))/2)%3  );
        // println!("1: {}", ((c + m)*255.0)  );
        // println!("2: {}", ((c*(1.0 - ((((vals[0] as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0)  );
        // println!("3: {}", ((m)*255.0)  );
        temp[((1 +      (vals[0]/60))/2)%3] = ((c + m)*255.0).round() as u8; // 0 1 1 2 2 0
        temp[(1 + 2*    (vals[0]/60))%3   ] = ((c*(1.0 - ((((vals[0] as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0).round() as u8; // 1 0 2 1 0 2
        temp[((4 +      (vals[0]/60))/2)%3] = ((m)*255.0).round() as u8; // 2 2 0 0 1 1
        // println!("{:?}", temp);
        Ok(Rgb::from(temp))
        // unimplemented!()

    } else if inp.get(0..3) == Some("hsv") {
        let t_vals: Vec<f64> = inp.split(|c| c == ' '||c == '{'||c == '}').filter_map(|x| x.parse::<f64>().ok()).collect();
        let vals = [(t_vals[0]*359.0) as usize, (t_vals[1]*100.0) as usize, (t_vals[2]*100.0) as usize];

        let c = ((vals[1]*vals[2]) as f64) / 10000.0;
        let m = (vals[2] as f64 / 100.0) - c;

        // println!("{:?}", vals);

        // temp[vals[0]/60] // 0 1 2 3 4 5 6
        //
        // 1 0 2 1 0 2
        // (1 + 2x)%3
        //
        // 0 1 1 2 2 0
        // (3x)%3
        //
        // 2 2 0 0 1 1
        // ((2 + x)/2)%3
        //
        // println!("{} {}", c, m);
        // println!("{}", ((1 +      (vals[0]/60))/2)%3  );
        // println!("{}", (1 + 2*    (vals[0]/60))%3     );
        // println!("{}", ((4 +      (vals[0]/60))/2)%3  );
        // println!("1: {}", ((c + m)*255.0)  );
        // println!("2: {}", ((c*(1.0 - ((((vals[0] as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0)  );
        // println!("3: {}", ((m)*255.0)  );
        temp[((1 +      (vals[0]/60))/2)%3] = ((c + m)*255.0).round() as u8; // 0 1 1 2 2 0
        temp[(1 + 2*    (vals[0]/60))%3   ] = ((c*(1.0 - ((((vals[0] as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0).round() as u8; // 1 0 2 1 0 2
        temp[((4 +      (vals[0]/60))/2)%3] = ((m)*255.0).round() as u8; // 2 2 0 0 1 1
        // println!("{:?}", temp);
        Ok(Rgb::from(temp))
    } else {
        vals = inp.split(|c| c == ' '||c == '{'||c == '}').filter_map(|x| x.parse::<f64>().ok()).collect();
        // println!("{:?}", vals);
        if vals.iter().sum::<f64>() < 4.0 {
            vals.iter_mut().for_each(|x| *x *= 255.0);
        }
        // println!("{:?}", vals);
        temp[0] = vals[0].clamp(0.1f64, 255.1f64).round() as u8;
        temp[1] = vals[1].clamp(0.1f64, 255.1f64).round() as u8;
        temp[2] = vals[2].clamp(0.1f64, 255.1f64).round() as u8;
        // println!("{:?}", temp);
        let a = Rgb::from_slice(&temp[0..3]);
        Ok(*a)
    }
}

pub fn analyse(inp: &str) -> Result<(impl Iterator<Item = String>, String), Box<dyn Error>> {

    let mut zipper: zip::ZipArchive<std::fs::File>;
    let mut writer: Vec<u8> = vec![];
    let file:       zip::read::ZipFile;
    let f:          std::fs::File;


    f = File::open(format!("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3/save games/{}.v3", inp))?;

    zipper  = zip::ZipArchive::new(f)?;
    file    = zipper.by_name("gamestate")?;

    let mut file = file;
    io::copy(&mut file, &mut writer)?;
    std::mem::drop(&mut file);
    match fs::create_dir("temp") {
        Err(e) if {&e.kind() != &io::ErrorKind::AlreadyExists} => return Err(Box::new(e)),
        _ => {}
    }

    match fs::remove_file(format!("temp/{}.txt", inp)) {
        Err(e) if {&e.kind() != &io::ErrorKind::NotFound} => return Err(Box::new(e)),
        _ => {}
    }
    fs::write(format!("temp/{}.txt", inp), &writer)?;

    get_file(&format!("temp/{}.txt", inp), false)
}
pub fn get_file(inp: &str, preamble: bool) -> Result<(impl Iterator<Item = String>, String), Box<dyn Error>> {
    let file;
    if preamble {
        file = File::open(format!("/mnt/c/Steam/steamapps/common/Victoria 3/{}", inp))?
    } else {
        file = File::open(inp)?
    }
    Ok((BufReader::new(file).lines().filter_map(|x| x.ok()).map(|x| x.trim_start_matches("\u{feff}").to_owned()), inp.split('/').last().unwrap().to_owned()))
}

pub fn get_glob(parent: &str, filetype: &str) -> Result<Paths, PatternError> {
    glob(&format!("/mnt/c/Steam/steamapps/common/Victoria 3/{}/*.{}", parent, filetype))
}

pub fn get_provinces(flip: bool, shrink: Option<f64>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut ret = image::open("/mnt/c/Steam/steamapps/common/Victoria 3/game/map_data/provinces.png")?;
    if let Some(a) = shrink {
        ret = ret.resize(((a * ret.width() as f64) / 100.0) as u32, u32::MAX, image::imageops::FilterType::Nearest);
    }
    if flip {
        Ok(ret.flipv().into_rgb8())
    } else {
        Ok(ret.into_rgb8())
    }
}

pub fn delete() -> Result<(), Box<dyn Error>> {
    match fs::remove_dir_all("temp") {
        Err(e) if {&e.kind() != &io::ErrorKind::NotFound} => return Err(Box::new(e)),
        _ => {}
    }
    fs::create_dir("temp")?;
    Ok(())
}

pub fn save(spath: &str, fname: &str, thing: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(), Box<dyn Error>> {
    // let a: String = spath.split('/').zip(spath.split('/').skip(1)).map(|x| format!("{}/", x.0)).collect();
    // let a: String = a.chars().zip(a.chars().skip(1)).map(|x| x.0).collect();
    match fs::create_dir_all(spath) {
        Err(e) if {&e.kind() != &io::ErrorKind::AlreadyExists} =>  return Err(Box::new(e)),
        _ => {}
    }
    match fs::remove_file(&format!("{spath}/{fname}")) {
        Err(e) if {&e.kind() != &io::ErrorKind::NotFound} => return Err(Box::new(e)),
        _ => {}
    }
    thing.save(format!("{spath}/{fname}"))?;

    Ok(())
}