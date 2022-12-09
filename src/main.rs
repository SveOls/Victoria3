// #![feature(iterator_try_collect, result_option_inspect, type_alias_impl_trait, is_some_and)]
#![feature(iterator_try_collect, iter_next_chunk)]
#![allow(dead_code, unreachable_code)]

use error::VicError;

mod data;
mod draw;
mod error;
mod scanner;
mod testing;
mod utilities;
mod vicapp;
mod wrappers;

fn main() -> Result<(), VicError> {



    vicapp::run()?;

    // arable land
    // let mut d = HashMap::new();
    // for (i, area) in _data.state_area() {
    //     if let Some(a) = i.arable_land() {
    //         d.insert(i.id().1, a as f64 / area as f64);
    //     }
    // }



    // find best polynomial
    // let mut variance;
    // let mut lowest = (f64::MAX, 0.0);
    // for test in 1..200 {
    //     variance = 0.0;
    //     let t_2d = t_d
    //         .iter()
    //         .enumerate()
    //         .map(|(a, (_, b))| (a as f64, b.powf(test as f64 / 100.0)))
    //         .collect::<Vec<(f64, f64)>>();
    //     let maxxer = t_2d.iter().fold(0f64, |a, &b| a.max(b.1));
    //     let minner = t_2d.iter().fold(1f64, |a, &b| a.min(b.1));
    //     let t_3d = t_2d
    //         .into_iter()
    //         .map(|(a, b)| (a, (b - minner) / (maxxer - minner)))
    //         .collect::<Vec<(f64, f64)>>();
    //     for (x, y) in &t_3d {
    //         variance += (y - (x / (t_3d.len() - 1) as f64)).abs();
    //     }
    //     if variance < lowest.0 {
    //         lowest = (variance, test as f64 / 100.0)
    //     }
    // }


    Ok(())
}
