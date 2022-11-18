

// use itertools::Itertools;
use regex::Regex;
use image::Rgb;

use std::error::Error;
use std::collections::HashMap;

mod strategic;

// use statetemplates::StateTemplate;
use strategic::{StrategicRegion, statetemplates::StateTemplate};
use super::analyse;

#[derive(Debug, Default)]
pub struct Map {
    index_color:    Vec<(Rgb<u8>, usize)>,
    provinces:      HashMap<Rgb<u8>, usize>,
    regions:        Vec<StrategicRegion>,
    countries:      HashMap<String, Rgb<u8>>,
    unorganized:    Vec<Rgb<u8>>,
}


impl Map {
    pub fn new() -> Result<Self, Box<dyn Error>> {

        let mut ret = Self::default();

        let mut strategic_regions = Vec::new();
        let mut state_templates = Vec::new();

        for entry in analyse::get_glob("game/map_data/state_regions", "txt")? {

            // break;

            let (mut fileiter, name) = analyse::get_file(entry?.to_str().unwrap(), false)?;


            while let Some(mut a) = StateTemplate::new(&mut fileiter, name == "99_seas.txt")? {

                a.set_id(state_templates.len());

                state_templates.push(Some(a));
            }
        }

        let mut offset = 1;
        for entry in analyse::get_glob("game/common/strategic_regions", "txt")? {

            // break;

            let (mut fileiter, name) = analyse::get_file(entry?.to_str().unwrap(), false)?;


            // this is a bit of a mess. A vector of states is passed in so they can be assigned to the new strategic regions as they're created.
            // in addition, "offset" is the ID of the first province in the new strategic region.
            while let Some(mut a) = StrategicRegion::new(&mut fileiter, &mut state_templates, offset, name == "water_strategic_regions.txt")? {
                a.set_id(strategic_regions.len());
                offset += a.size();
                strategic_regions.push(a);
            }
        }

        let mut countries = HashMap::new();
        let id_reg = Regex::new(r#"^([A-Z]{3}) = \{|^\tcolor = ([a-z\{\} .0-9]+)"#)?;

        for entry in analyse::get_glob("game/common/country_definitions", "txt")? {

            // break;

            let (mut data, _) = analyse::get_file(entry?.to_str().unwrap(), false)?;

            let mut tag = String::new();

            while let Some(a) = data.next() {
                for b in id_reg.captures_iter(&a) {
                    if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                        tag = c.to_owned();
                    }
                    if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                        if tag.chars().count() != 3 {
                            panic!();
                        }
                        countries.insert(tag, analyse::to_rgb(c)?);
                        tag = String::new();
                    }
                }
            }
        }
        // println!("{:?}", countries);

        // let (mut data, _) = analyse::get_file("game/map_data/default.map", true)?;

        // let id_reg = Regex::new(r#"x([0-9A-Fa-f]{6})"#)?;

        // let mut lakes = Vec::new();

        // while let Some(a) = data.next() {
        //     for b in id_reg.captures_iter(&a) {
        //         if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
        //             lakes.push(Rgb::from([u8::from_str_radix(&c[0..2], 16).unwrap(), u8::from_str_radix(&c[2..4], 16).unwrap(), u8::from_str_radix(&c[4..6], 16).unwrap()]));
        //         }
        //     }
        // }



        // flipv so iterating through goes left-right, down-up, as opposed to left-right, up-down. That way, assigning province index and ID can be done iteratively,
        // as provinces in each state are enumerated based on whick is encountered first when scanning through the image, pixel by pixel, in this way.
        let img = analyse::get_provinces(true, None)?;


        let mut provinces: Vec<(Option<Rgb<u8>>, usize)> = vec![(None, 0); strategic_regions.iter().map(|x| x.size()).sum::<usize>() + 1];

        let mut colors = HashMap::new();

        let mut unorganized = Vec::new();


        for (_, &img_pixel) in img.pixels().enumerate() {

            if let Some(&val) = colors.get(&img_pixel) {
                // println!("{:?}", provinces[val]);
                provinces[val as usize].1 += 1;
                continue;
            }

            let mut change = false;
            'outer: for region in &strategic_regions {
                if let Some((offset, statesize)) = region.get_number(img_pixel)? {
                    change = true;

                    for i in offset..offset+statesize {
                        if provinces[i].0.is_none() {

                            provinces[i].0 = Some(img_pixel);
                            provinces[i].1 += 1;
                            colors.insert(img_pixel, i);

                            break 'outer;
                        }
                    }
                    unreachable!();
                }
            }
            if !change {
                provinces.push((Some(img_pixel), 1));
                colors.insert(img_pixel, provinces.len() - 1);
                unorganized.push(img_pixel);
            }
        }

        provinces[0] = (Some(Rgb::from([0; 3])), 0);
        ret.index_color = provinces.iter().map(|&x| (x.0.unwrap(), x.1)).map(|x| (Rgb::from(x.0), x.1)).collect();
        ret.provinces = colors;
        ret.unorganized = unorganized;
        ret.countries = countries;
        ret.regions = strategic_regions;

        Ok(ret)
    }
    pub fn get_strat_color(&self, prov: Rgb<u8>) -> Option<Rgb<u8>> {
        for region in &self.regions {
            if let Some(color) = region.get_color(prov, None) {
                return Some(color)
            }
        }
        None
        // Err(Box::new(io::Error::new(io::ErrorKind::Other, "couldn't find strat color")))
    }
    /// Returns state name if province belongs to known state template.
    ///
    /// Returns "UNORGANIZED" if province is known (located on province map), but was not found in a state template.
    ///
    /// Returns None if it wasn't found in provinces.png nor in any state template.
    pub fn get_state_name(&self, color: Rgb<u8>) -> Option<&str> {
        // self.regions.iter()  -> iterate over each region
        // .map(|x| f(x))       -> gets Option<StateTemplate> of each state in region and turns them into Option<&str>.
        // .chain(....)         -> chains "UNORGANIZED" to the end of the iterator if "self.unorganized" contains "color".
        // .filter_man(....)    -> Options are unpacked and Nones are ignored
        // .next()              -> first &str in iterator is returned.
        //
        let ret = self.regions.iter()
            .map(
                |region| region.get_state(color).and_then(|x| Some(x.name()))
            ).chain(
                [Some("UNORGANIZED")].into_iter().filter(|_| self.unorganized.contains(&color))
            ).filter_map(|name| name).next();

        /*
        let mut ret2 = None;
        for region in &self.regions {
            if let Some(state) = region.get_state(color) {
                ret2 = Some(state.name())
            }
        }
        if self.unorganized.contains(color) {
            ret2 = Some("UNORGANIZED");
        }
        if ret != ret2 {
            panic!();
        }
        */


        /*
        match self.regions.iter().filter_map(|region| region.get_state(color)).next() {
            Some(state) => {
                Some(state.name())
            }
            None if self.unorganized.contains(color) => {
                Some("UNORGANIZED")
            }
            None => None
        }
        // not country. this means Color province is not assigned to a state.
        */

        ret
    }
    ///
    pub fn to_index(&self, inp: Rgb<u8>) -> Option<usize> {
        self.provinces.get(&inp).cloned()
    }
    pub fn get_country_color(&self, tag: &str) -> Option<Rgb<u8>> {
        self.countries.get(tag).cloned()
    }
    pub fn area(&self, provs: &Vec<usize>) -> usize {
        provs.iter().map(|&x| self.index_color[x].1).sum()
    }
    pub fn get_region(&self, id: usize) -> Option<&StrategicRegion> {
        self.regions.get(id)
    }
    pub fn regions(&self) -> &Vec<StrategicRegion> {
        &self.regions
    }
    pub fn get_state(&self, id_1: usize) -> Option<&StateTemplate> {
        self.regions.iter().filter_map(|x| x.get_state(self.index_color[id_1].0)).next()
    }
}




