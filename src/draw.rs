

use super::map;
use super::utilities;
use super::save::Save;


use std::collections::{HashMap, HashSet};
use std::default;
use std::path::Path;
use std::path::PathBuf;

use crate::data::Info;
use crate::error::VicError;
use crate::wrappers::{ImageWrap, ColorWrap};

use image::Rgb;

#[derive(Debug)]
pub enum DrawMap {
    StrategicRegion,
    StateTemplate,
    SaveStates,
    SaveCountries,
    SaveStatesData,
    StateTemplateData,
}

pub enum Coloring {
    StateTemplates,
    SaveStates,
    SaveCountries,
    Provinces,
    None,
}

impl Default for Coloring {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default)]
pub struct MapDrawer {
    resize:         Option<f64>,
    numerator:      Option<HashMap<usize, f64>>,
    denominator:    Option<HashMap<usize, f64>>,
    /// these two hashmaps might hog space. idk.
    lines:          Coloring,
    premade_lines:  HashMap<Coloring, ImageWrap>,
    color_map:      Coloring,
    premade_color:  HashMap<Coloring, ImageWrap>,
    /// color of ocean
    color:          Option<ColorWrap>,
    /// color of data - overrides the default.
    othercolor:     Option<ColorWrap>,
    province_path:  Option<PathBuf>,
    darkmode:       bool,
}

impl MapDrawer {
    pub fn resize(&mut self, inp: f64) {
        self.resize = Some(inp)
    }
    /// generally a religion, culture, or something else.
    pub fn set_numerator(&mut self, inp: HashMap<usize, f64>) {
        self.numerator = Some(inp)
    }
    /// generally per capita or per area
    pub fn set_denominator(&mut self, inp: HashMap<usize, f64>) {
        self.denominator = Some(inp)
    }
    pub fn set_color(&mut self, inp: ColorWrap) {
        self.color = Some(inp)
    }
    pub fn set_unassigned_color(&mut self, inp: ColorWrap) {
        self.othercolor = Some(inp)
    }
    pub fn darkmode(&mut self, inp: bool) {
        self.darkmode = inp
    }
    pub fn set_lines(&mut self, inp: Coloring) {
        self.lines = inp
    }
    pub fn set_color_map(&mut self, inp: Coloring) {
        self.color_map = inp
    }
    pub fn clear(self) -> Self {
        Self {
            premade_lines: self.premade_lines,
            premade_color: self.premade_color,
            ..Self::default()
        }
    }
    pub fn draw(&mut self, info: &Info) {

    }
}



impl DrawMap {
    /// resize: 100 = normal, 10 = width and height is 10%, etc.
    ///
    /// progress frequency: vertical lines between each update.
    ///
    /// debug: whether the province map is saved or not.
    ///
    /// sav: save data, not required for StrategicRegion or StateTemplate, but required for others.
    ///
    /// unassigned_color: if function can't find a color OR the province is a lake or ocean, this is used.
    ///
    /// versions: province map, line map, recolored, recolored with lines. Will always generate a map, even if all false, just not save it.
    pub fn draw(self, path: &Path, versions: &[bool; 4], inp: &map::Map, data: Option<(HashMap<usize, f64>, Option<ColorWrap>, bool)>, resize: Option<f64>, progress_frequency: Option<u32>, sav: Option<&Save>, unassigned_color: Option<Rgb<u8>>) -> Result<(), VicError> {

        let mut savedcolors: HashMap<Rgb<u8>, Rgb<u8>> = HashMap::new();
        let mut statecol: HashMap<String, Rgb<u8>> = HashMap::new();
        let mut tags: HashMap<String, Rgb<u8>> = HashMap::new();
        let mut ids: HashMap<usize, Rgb<u8>> = HashMap::new();

        let mut pathpath = std::path::PathBuf::new();
        pathpath.push("/mnt/c/Steam/steamapps/common/Victoria 3/");

        let province_map = crate::wrappers::ImageWrap::new(path.to_path_buf(), resize)?;
        let (width, height) = province_map.dimensions();

        let mut new_map = province_map.clone();

        let datacol;
        let defcol;
        if let Some((_, Some(a), tre)) = data {
            datacol = a;
            if tre {
                defcol = ColorWrap::Rgb(Rgb::from([0xFF, 0xFF, 0xFF]));
            } else {
                defcol = ColorWrap::Rgb(Rgb::from([0x00, 0x00, 0x00]));
            }
        } else {
            defcol = ColorWrap::Rgb(Rgb::from([0x0, 0x00, 0x00]));
            datacol = ColorWrap::Rgb(Rgb::from([0xFF, 0xFF, 0xFF]));
        }

        let errorthrow = |color: Rgb<u8>, i: u32, width: u32| -> VicError {
            VicError::Other(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}: failed at color: {:X}{:X}{:X} and coordinate: {}x, {}y", self, color[0], color[1], color[2], i%width, i/width))))
        };

        for (i,  color) in new_map.pixels_mut().enumerate().map(|x| (x.0 as u32, x.1)) {
            if let Some(l) = progress_frequency {
                if i%(l*width) == 0 {
                    println!("{}\t/{}\t{:.0}%", i/width, height, (100.0*((i/width) as f64))/(height as f64));
                }
            }

            if let Some(a) = savedcolors.get(&color) {

                *color = *a;

            } else {
                let draw;
                if let Some(index) = inp.to_index(*color) {
                    match self {
                        DrawMap::StrategicRegion => {
                            if let Some(st_color) = inp.get_strat_color(*color) {
                                draw = st_color;
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        DrawMap::StateTemplate => {
                            // IF color is bound to a state { keep drawing } ELSE { return Error }
                            if let Some(state) = inp.col_index_to_state(index) {
                                if let Some(&col) = statecol.get(state.name()) {
                                    draw = col;
                                } else if state.is_ocean() {
                                    draw = unassigned_color.unwrap_or(*color);
                                } else {
                                    statecol.insert(state.name().to_owned(), *color);
                                    draw = *color;
                                }
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        DrawMap::StateTemplateData => {
                            // IF color is bound to a state { keep drawing } ELSE { return Error }
                            if let Some(state) = inp.col_index_to_state(index) {
                                if let Some(&col) = statecol.get(state.name()) {
                                    draw = col;
                                } else if state.is_ocean() {
                                    draw = unassigned_color.unwrap_or(*color);
                                } else {
                                    let colore;
                                    if let Some(dat) = &data {
                                        if let Some(factor) = dat.0.get(&state.id().1) {
                                            colore = defcol.scale_to(&datacol, *factor).unravel();
                                        } else {
                                            colore = Rgb::from([0xFF, 0xFF, 0xFF]);
                                        }
                                    } else {
                                        colore = Rgb::from([0xFF, 0xFF, 0xFF]);
                                    }
                                    statecol.insert(state.name().to_owned(), colore);
                                    draw = colore
                                }
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        DrawMap::SaveStates => {
                            if let Some(id) = sav.unwrap().get_owners(index).map(|(s, _)| s.id()) {
                                if let Some(&newc) = ids.get(&id) {
                                    draw = newc
                                } else {
                                    ids.insert(id, *color);
                                    draw = *color
                                }
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        DrawMap::SaveStatesData => {
                            if let Some(id) = sav.unwrap().get_owners(index).map(|(s, _)| s.id()) {
                                if let Some(&newc) = ids.get(&id) {
                                    draw = newc
                                } else {
                                    let colore;
                                    if let Some(dat) = &data {
                                        if let Some(factor) = dat.0.get(&id) {
                                            if dat.2 {
                                                colore = datacol.scale_to(&defcol, *factor).unravel();
                                            } else {
                                                colore = datacol.scale_to(&defcol, *factor).unravel();
                                            }
                                        } else {
                                            colore = defcol.unravel();
                                        }
                                    } else {
                                        colore = defcol.unravel();
                                    }
                                    ids.insert(id, colore);
                                    draw = colore
                                }
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        DrawMap::SaveCountries => {
                            if let Some(ctag) = sav.unwrap().get_owners(index).map(|(_, c)| c.tag()) {
                                if let Some(&newc) = tags.get(ctag) {
                                    draw = newc
                                } else if let Some(newc) = inp.get_country_color(&ctag) {
                                    draw = newc;
                                } else {
                                    draw = *color;
                                }
                                tags.insert(ctag.clone(), draw);

                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        }
                        // _ => panic!()
                    }
                } else {
                    return Err(errorthrow(*color, i, width));
                }
                savedcolors.insert(*color, draw);
                *color = draw;
            }
        }
        let (a, b) = self.border(&new_map, versions[3]);



        let nam;
        let pat;
        match self {
            DrawMap::StrategicRegion => {
                nam = "Strategic";
                pat = "output/map";
            }
            DrawMap::StateTemplate => {
                nam = "States";
                pat = "output/map";
            }
            DrawMap::StateTemplateData => {
                nam = "StatesData";
                pat = "output/map";
            }
            DrawMap::SaveStates => {
                nam = "States";
                pat = "output/save";
            }
            DrawMap::SaveCountries => {
                nam = "Country";
                pat = "output/save";
            }
            DrawMap::SaveStatesData => {
                nam = "StateData";
                pat = "output/save";
            }
        }
        if versions[0] {
            utilities::save(pat, &format!("{nam}_debug.png"), province_map)?;
        }
        if versions[1] {
            utilities::save(pat, &format!("{nam}_emptylines.png"), a)?;
        }
        if versions[2] {
            utilities::save(pat, &format!("{nam}.png"), new_map)?;
        }
        if versions[3] {
            utilities::save(pat, &format!("{nam}_lines.png"), b.unwrap())?;
        }
        Ok(())
    }

    fn border(&self, inp: &ImageWrap, inp_with_borders: bool) -> (ImageWrap, Option<ImageWrap>) {

        // let (width, height) = inp.dimensions();

        // dir is clockwise. Center is zero, north is 1, west is 4.
        // let tclos = |x: u32, y: u32, dir: u32| -> Option<&Rgb<u8>> {
        //     match dir {
        //         1 if y == 0 => inp.get_pixel_checked(x, u32::MAX),
        //         4 if x == 0 => inp.get_pixel_checked(u32::MAX, y),
        //         // weird math just ensures x and y is shifted appropriately
        //         0..=4       => inp.get_pixel_checked(x + (((dir + 1)%4)/3 - ((dir + 3)%4)/3 + ((dir + 7)%8)/7), y + (((dir%4)/3) - ((dir + 2)%4)/3)),
        //         _ => panic!()
        //     }
        // };


        let mid     = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_add(0));
        let above   = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_add(1));
        let right   = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_sub(1));
        let below   = inp.pixels_offset(|x| x.checked_sub(1), |y| y.checked_add(0));
        let left    = inp.pixels_offset(|x| x.checked_add(1), |y| y.checked_add(0));

        let mut blank = inp.new_empty(ColorWrap::from(Rgb::from([255, 255, 255])));

        //Zips a mutable iterator over a wite map with:
        //an iterator over neighboring pixels, folding into True if all pixels are equal, and False if they are not.
        {
            let mut temp = blank
                .pixels_mut()
                .zip(mid.zip(above.zip(right.zip(below.zip(left))))
                .map(|x| [x.0, x.1.0, x.1.1.0, x.1.1.1.0, x.1.1.1.1]).map(|x| x.iter().filter_map(|&x| x).collect::<HashSet<_>>().len() == 1));

            for (pixel, condition) in temp {
                if !condition {
                    *pixel = Rgb::from([0u8, 0, 0]);
                }
            }
        }
        if inp_with_borders {
            let mut neu = inp.clone();

            for (pix, refr) in neu.pixels_mut().zip(blank.pixels()) {
                if refr.0 == [0, 0, 0] {
                    *pix = *refr
                }
            }

            (blank, Some(neu))
        } else {
            (blank, None)
        }
    }
}

