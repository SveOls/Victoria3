// #![feature(iterator_try_collect, result_option_inspect, type_alias_impl_trait, is_some_and)]
#![feature(iterator_try_collect)]
#![allow(dead_code, unreachable_code)]


use std::collections::HashMap;
use std::path::PathBuf;

use error::VicError;
use image::Rgb;

mod utilities;
mod save;
mod app;
mod map;
mod error;
mod data;
mod draw;
mod testing;
mod scanner;
mod wrappers;

use draw::DrawMap;

fn main() -> Result<(), VicError> {


    // let a = [50, 100, 200];
    // let max = 200;
    // let v = 200.0 / 255.0;
    // let s = 150.0/200.0;
    // let gg = false;
    // let test = |x: u8, f: f64| -> u8 {
    //     if gg {
    //         ((x as f64 + ((max - x) as f64 * (1.0 - f))) / ((1.0 - f)*(v - 1.0) + 1.0)) as u8
    //     } else {
    //         ((x as f64 - ((max - x) as f64 * (1.0 - f) * ((1.0 - s)/s))) * f) as u8
    //     }
    // };

    // println!("{}", test(a[0], 0.0));
    // println!("{}", test(a[1], 0.0));
    // println!("{}", test(a[2], 0.0));
    // println!("{}", test(a[0], 0.5));
    // println!("{}", test(a[1], 0.5));
    // println!("{}", test(a[2], 0.5));
    // println!("{}", test(a[0], 1.0));
    // println!("{}", test(a[1], 1.0));
    // println!("{}", test(a[2], 1.0));
    // panic!();


    app::run()?;

    panic!();

    let mut savelocation = PathBuf::new();
    savelocation.push("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3/");
    let mut gamelocation = PathBuf::new();
    gamelocation.push("/mnt/c/Steam/steamapps/common/Victoria 3/");


    println!("game analysis");

    let data = map::Map::new(&gamelocation)?;

    // panic!();

    println!("save analysis");
    // use std::time::{Duration, Instant};
    // let start = Instant::now();
    let  stuff = save::Save::new(&savelocation)?;
    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    let sea_color = Some(Rgb::from([0, 100, 200]));
    let progress_frequency = Some(300);
    let all_maps = [false, true, true, true];

    println!("map drawing");

    // for (&id, culture) in stuff.cultures() {
    //     println!("\n{}", culture.name());
    //     for (tag, (culpop, totpop)) in stuff.country_cultures(id)? {
    //         if culpop > 0 {
    //             println!("{}\t{} / {}", tag, culpop, totpop);
    //         }
    //     }
    // }


    let mut d = HashMap::new();
    let c = Some(Rgb::from([0x0, 0xFF, 0x7F]));
    for culture in stuff.cultures() {
        if culture.id() != 11 {
            continue;
        }
        for state in stuff.states() {
            let a = state.religion_pop("jewish")?;
            d.insert(state.id(), a.0 as f64 / a.1 as f64);
        }
        println!("{:?}", d);
        let max = d.values().fold(0f64, |a, &b| a.max(b));
        d = d.into_iter().map(|(a, b)| (a, b/max)).collect();
        break;
    }

    let mut d = HashMap::new();
    for (i, area) in data.state_area() {
        if let Some(a) = i.arable_land() {
            d.insert(i.id().1, a as f64 / area as f64);
        }
    }

    let max = d.values().fold(0f64, |a, &b| a.max(b));
    let min = d.values().fold(1f64, |a, &b| a.min(b));
    d = d.into_iter().map(|(a, b)| (a, (b-min)/(max-min))).collect();

    let mut t_d = d.iter().map(|(a, b)| (*a, *b)).collect::<Vec<(usize, f64)>>();
    t_d.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let t_d = t_d.iter().enumerate().map(|(a, (_, b))| (a as f64, *b)).collect::<Vec<(f64, f64)>>();

    // for i in &t_d {
    //     println!("{i:?}")
    // }
    // panic!();

    let mut variance;
    let mut lowest = (f64::MAX, 0.0);
    for test in 1..200 {
        variance = 0.0;

        let t_2d = t_d.iter().enumerate().map(|(a, (_, b))| (a as f64, b.powf(test as f64 / 100.0))).collect::<Vec<(f64, f64)>>();
        let maxxer = t_2d.iter().fold(0f64, |a, &b| a.max(b.1));
        let minner = t_2d.iter().fold(1f64, |a, &b| a.min(b.1));
        let t_3d = t_2d.into_iter().map(|(a, b)| (a, (b-minner)/(maxxer-minner))).collect::<Vec<(f64, f64)>>();

        // println!("{minner} {maxxer} to {}", t_3d[0].1);

        // for i in &t_3d {
        //     println!("{i:?}");
        // }
        // panic!();


        for (x, y) in &t_3d {
            variance += (y - (x / (t_3d.len() - 1) as f64)).abs();
        }
        if variance < lowest.0 {
            lowest = (variance, test as f64 / 100.0)
        }
        // println!(">{lowest:?}")
    }

    let mut d = d.into_iter().map(|(a, b)| (a, b.powf(lowest.1))).collect::<HashMap<usize, f64>>();
    // d.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());



    // DrawMap::StrategicRegion.   draw(&all_maps, &data, None,         None, progress_frequency, None,          sea_color)?;
    // DrawMap::StateTemplate.     draw(&all_maps, &data, None,         None, progress_frequency, None,          sea_color)?;
    // DrawMap::SaveCountries.     draw(&all_maps, &data, None,         None, progress_frequency, Some(&stuff),  sea_color)?;
    // DrawMap::SaveStates.        draw(&all_maps, &data, None,         None, progress_frequency, Some(&stuff),  sea_color)?;
    // DrawMap::SaveStatesData.    draw(&all_maps, &data, Some((d, c)), None, progress_frequency, Some(&stuff),  sea_color)?;
    // DrawMap::StateTemplateData. draw(&all_maps, &data, Some((d, c)), None, progress_frequency, Some(&stuff),  sea_color)?;
    // panic!();


    Ok(())
}

