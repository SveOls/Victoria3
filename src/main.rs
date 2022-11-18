#![feature(iterator_try_collect)]
#![allow(dead_code, unreachable_code)]

use std::error::Error;
use image::Rgb;

mod analyse;
mod save;
mod map;
mod draw;
mod testing;

use draw::DrawMap;

fn main() -> Result<(), Box<dyn Error>> {





    testing::tester()?;




    panic!();

    println!("game analysis");

    let data = map::Map::new()?;


    println!("save analysis");
    let stuff = save::Save::new("great britain_1836_01_01")?;

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
    // println!("{:?}", stuff.area(366, &data));
    // println!("{:?}", stuff.area(844, &data));
    // println!("{:?}", stuff.area(816, &data));
    // panic!();


    DrawMap::StrategicRegion.   draw(&all_maps, &data, None, progress_frequency, None,          sea_color)?;
    DrawMap::StateTemplate.     draw(&all_maps, &data, None, progress_frequency, None,          sea_color)?;
    DrawMap::SaveCountries.     draw(&all_maps, &data, None, progress_frequency, Some(&stuff),  sea_color)?;
    DrawMap::SaveStates.        draw(&all_maps, &data, None, progress_frequency, Some(&stuff),  sea_color)?;
    // panic!();

    // draw::strategic_regions(&data, 100, progress_frequency, false)?;
    // draw::state_regions(&data, 100, progress_frequency, false)?;
    // draw::state_countries(&data, &stuff, sea_color, 100, progress_frequency, false)?;
    // draw::states(&data, &stuff, sea_color, 100, progress_frequency, false)?;





    // println!("{} {} {}", pops.len(), states.len(), countries.len());


    analyse::delete()?;

    Ok(())
}

