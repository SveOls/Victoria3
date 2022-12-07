use super::data::map;
use super::data::save::Save;
use super::utilities;

use std::collections::{HashMap, HashSet};
// use std::default;
use std::path::Path;
use std::path::PathBuf;

use crate::data::Info;
use crate::error::VicError;
use crate::wrappers::{ColorWrap, ImageWrap};

use image::Rgb;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
    resize: Option<f64>,
    /// generally a culture, religion, or other Mappable Value
    numerator: Option<HashMap<usize, usize>>,
    /// generally area, total population, or other value to map the Numerator against (e.g. Pop / Area)
    denominator: Option<HashMap<usize, usize>>,
    /// these two hashmaps might hog space. idk.
    lines: Coloring,
    premade_lines: HashMap<Coloring, ImageWrap>,
    color_map: Coloring,
    premade_color: HashMap<Coloring, ImageWrap>,
    /// color of ocean
    sea_color: Option<ColorWrap>,
    /// color of data.
    data_color: Option<ColorWrap>,
    provice_map_path: Option<PathBuf>,
    default_color: ColorWrap,
    /// memoization of province color to its color and owner. Separated options are for cases where
    /// the color is erased, but the other data is still relevant.
    owners: HashMap<Rgb<u8>, Option<([Option<Rgb<u8>>; 3], Option<[usize; 2]>)>>,
}

impl MapDrawer {
    pub fn resize(&mut self, inp: f64) {
        self.resize = Some(inp)
    }
    /// generally a religion, culture, or something else.
    pub fn set_numerator(&mut self, inp: Option<HashMap<usize, usize>>) {
        self.numerator = inp
    }
    /// generally per capita or per area
    pub fn set_denominator(&mut self, inp: Option<HashMap<usize, usize>>) {
        self.denominator = inp
    }
    pub fn set_sea_color(&mut self, inp: ColorWrap) {
        self.sea_color = Some(inp)
    }
    pub fn set_color(&mut self, inp: Option<ColorWrap>) {
        self.data_color = inp
    }
    pub fn darkmode(&mut self, inp: bool) {
        if inp {
            self.default_color = ColorWrap::from(Rgb::from([0x00, 0x00, 0x00]))
        } else {
            self.default_color = ColorWrap::from(Rgb::from([0xFF, 0xFF, 0xFF]))
        }
    }
    pub fn set_lines(&mut self, inp: Coloring) {
        self.lines = inp
    }
    pub fn set_path(&mut self, inp: PathBuf) {
        self.provice_map_path = Some(inp)
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

    pub fn draw(&mut self, info: &Info, save_to: PathBuf, map_data: bool) -> Result<(), VicError> {
        self.draw_map_coloring(info)?;
        self.draw_lines_coloring(info)?;

        let mut saver = self.premade_color.get(&self.color_map).unwrap().clone();

        if map_data {
            let mut data: HashMap<usize, f64>;
            match (&mut self.numerator, &mut self.denominator) {
                (Some(num), Some(den)) => {
                    data = num
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, den.get(&k).map_or(0.0, |&x| v as f64 / x as f64)))
                        .collect()
                }
                (Some(num), None) => {
                    data = num
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k, v as f64))
                        .collect()
                }
                _ => return Err(VicError::named("no numerator when trying to map")),
            }
            let max = data.values().fold(0.0f64, |a, &b| a.max(b));
            let min = data.values().fold(1.0f64, |a, &b| a.min(b));

            data.values_mut()
                .for_each(|x| *x = (*x - min) / (max - min));

            let map = info.get_map()?;
            let save = info.get_save()?;

            let mut datar: HashMap<Rgb<u8>, Rgb<u8>> = HashMap::new();
            saver.pixels_mut().for_each(|x| {
                *x = datar.get(x).map(|x| *x).unwrap_or_else(|| {
                    datar.insert(
                        *x,
                        map.to_index(*x)
                            .and_then(|id| match self.color_map {
                                Coloring::None | Coloring::Provinces => unreachable!(),
                                Coloring::StateTemplates => {
                                    map.col_index_to_state(id).map(|s| s.id().1)
                                }
                                Coloring::SaveStates => save.get_owners(id).map(|y| y.0.id()),
                                Coloring::SaveCountries => save.get_owners(id).map(|y| y.1.id()),
                            })
                            .and_then(|id| data.get(&id))
                            .map_or_else(
                                || {
                                    if map.to_index(*x).and_then(|h| save.get_owners(h)).is_some() {
                                        self.default_color.unravel()
                                    } else {
                                        self.sea_color.map(|x| x.unravel()).unwrap_or(*x)
                                    }
                                },
                                |&a| {
                                    self.data_color
                                        .ok_or(VicError::named("no datacolor"))
                                        .unwrap()
                                        .scale_to(&self.default_color, a)
                                        .unravel()
                                },
                            ),
                    );
                    *datar.get(x).unwrap()
                })
            });
        }

        if self.lines != Coloring::None {
            saver
                .pixels_mut()
                .zip(self.premade_lines.get(&self.lines).unwrap().pixels())
                .filter(|&(_, &y)| y == Rgb::from([0, 0, 0]))
                .for_each(|(x, _)| *x = Rgb::from([0, 0, 0]))
        }

        crate::utilities::save3(save_to, "test.png", &saver)?;

        Ok(())
    }
    fn draw_lines_coloring(&mut self, info: &Info) -> Result<(), VicError> {
        if self.lines == Coloring::None {
            return Ok(());
        }
        let base_map = match self.premade_color.get(&self.lines) {
            Some(a) => a,
            None => {
                std::mem::swap(&mut self.lines, &mut self.color_map);
                self.draw_map_coloring(info)?;
                std::mem::swap(&mut self.lines, &mut self.color_map);
                self.premade_color.get(&self.lines).unwrap()
            }
        };
        let cross =
            base_map
                .pixels_offset(|x| x.checked_add(0), |y| y.checked_add(0))
                .zip(
                    base_map
                        .pixels_offset(|x| x.checked_add(0), |y| y.checked_add(1))
                        .zip(
                            base_map
                                .pixels_offset(|x| x.checked_add(0), |y| y.checked_sub(1))
                                .zip(
                                    base_map
                                        .pixels_offset(|x| x.checked_sub(1), |y| y.checked_add(0))
                                        .zip(base_map.pixels_offset(
                                            |x| x.checked_add(1),
                                            |y| y.checked_add(0),
                                        )),
                                ),
                        ),
                )
                .map(|x| [x.0, x.1 .0, x.1 .1 .0, x.1 .1 .1 .0, x.1 .1 .1 .1])
                .map(|x| x.iter().filter_map(|&x| x).collect::<HashSet<_>>().len() == 1);

        let mut blank = base_map.new_empty(ColorWrap::from(Rgb::from([0xFF, 0xFF, 0xFF])));

        let temp = blank.pixels_mut().zip(cross);

        for (pixel, condition) in temp {
            if !condition {
                *pixel = Rgb::from([0x00, 0x00, 0x00])
            }
        }

        self.premade_lines.insert(self.lines, blank);

        Ok(())
    }
    fn draw_map_coloring(&mut self, info: &Info) -> Result<(), VicError> {
        let province_map = self
            .premade_color
            .remove(&Coloring::Provinces)
            .unwrap_or(ImageWrap::new(self.get_path()?, self.resize)?);

        if self.premade_color.contains_key(&self.color_map) {
            return Ok(());
        }

        let mut new_map = province_map.clone();

        let mut memo = HashMap::new();

        let map = info.get_map()?;
        let save = info.get_save()?;

        for color in new_map.pixels_mut() {
            let updatedcolor;
            match self.owner_getter(color)? {
                Some((Some(newcolor), _)) => {
                    updatedcolor = newcolor;
                }
                Some((_, maybe_index)) => {
                    // maps Option<usize> to Option<result<option<usize>>>. Outermost option disappears upon unwrap_or_else,
                    // middle result is necessary for error handling, inner option is necessary for handling provinces with
                    // no owner (lakes, ocean). It then tries to find new "owners" (state, country) if the index is None.
                    let perhaps_index = maybe_index
                        .map(|x| Ok::<Option<usize>, VicError>(Some(x)))
                        .unwrap_or_else(|| {
                            let temp_owners = save
                                .get_owners(
                                    map.to_index(*color)
                                        .ok_or(VicError::named("no province index?"))?,
                                )
                                .map(|(x, y)| [x.id(), y.id()]);
                            self.owners.insert(*color, Some(([None; 3], temp_owners)));
                            match self.color_map {
                                Coloring::None | Coloring::Provinces => unreachable!(),
                                Coloring::SaveCountries => Ok(temp_owners.map(|x| x[1])),
                                Coloring::SaveStates => Ok(temp_owners.map(|x| x[0])),
                                Coloring::StateTemplates => Ok(temp_owners.map(|x| x[0])),
                            }
                        })?;
                    if let Some(index) = perhaps_index {
                        updatedcolor = *memo.entry(index).or_insert({
                            match self.color_map {
                                Coloring::StateTemplates => map
                                    .to_index(*color)
                                    .and_then(|x| map.col_index_to_state(x).and_then(|y| y.color()))
                                    .ok_or(VicError::named("Couldn't find state template color"))?,
                                Coloring::SaveStates => *color,
                                Coloring::SaveCountries => map
                                    .get_country_color_tag(save.get_country(index)?.tag())
                                    .unwrap_or(*color),
                                Coloring::None => unreachable!(),
                                Coloring::Provinces => unreachable!(),
                            }
                        });
                        self.owner_inserter(color, updatedcolor);
                    } else {
                        updatedcolor = self.sea_color.map(|x| x.unravel()).unwrap_or(*color);
                        self.owners.insert(*color, None);
                    }
                }
                None => updatedcolor = self.sea_color.map(|x| x.unravel()).unwrap_or(*color),
            }
            *color = updatedcolor;
        }
        self.premade_color.insert(Coloring::Provinces, province_map);
        self.premade_color.insert(self.color_map, new_map);
        Ok(())
    }

    fn owner_getter(
        &self,
        color: &Rgb<u8>,
    ) -> Result<Option<(Option<Rgb<u8>>, Option<usize>)>, VicError> {
        match self.color_map {
            Coloring::StateTemplates => Ok(self
                .owners
                .get(color)
                .map(|i| i.map(|(x, y)| (x[0], y.map(|z| z[0]))))
                .unwrap_or(Some((None, None)))),
            Coloring::SaveStates => Ok(self
                .owners
                .get(color)
                .map(|i| i.map(|(x, y)| (x[1], y.map(|z| z[0]))))
                .unwrap_or(Some((None, None)))),
            Coloring::SaveCountries => Ok(self
                .owners
                .get(color)
                .map(|i| i.map(|(x, y)| (x[2], y.map(|z| z[1]))))
                .unwrap_or(Some((None, None)))),
            Coloring::Provinces => Ok(Some((Some(*color), None))),
            Coloring::None => Err(VicError::temp()),
        }
    }
    fn owner_inserter(&mut self, color: &Rgb<u8>, updatedcolor: Rgb<u8>) {
        match self.color_map {
            Coloring::StateTemplates => self
                .owners
                .entry(*color)
                .and_modify(|a| a.iter_mut().for_each(|x| x.0[0] = Some(updatedcolor))),
            Coloring::SaveStates => self
                .owners
                .entry(*color)
                .and_modify(|a| a.iter_mut().for_each(|x| x.0[1] = Some(updatedcolor))),
            Coloring::SaveCountries => self
                .owners
                .entry(*color)
                .and_modify(|a| a.iter_mut().for_each(|x| x.0[2] = Some(updatedcolor))),
            Coloring::Provinces => unreachable!(),
            Coloring::None => unreachable!(),
        };
    }
    fn get_path(&self) -> Result<PathBuf, VicError> {
        self.provice_map_path
            .clone()
            .ok_or(VicError::named("no path"))
    }
}

#[derive(Debug)]
pub enum DrawMap {
    StrategicRegion,
    StateTemplate,
    SaveStates,
    SaveCountries,
    SaveStatesData,
    StateTemplateData,
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
    pub fn draw(
        self,
        path: &Path,
        versions: &[bool; 4],
        inp: &map::Map,
        data: Option<(HashMap<usize, f64>, Option<ColorWrap>, bool)>,
        resize: Option<f64>,
        progress_frequency: Option<u32>,
        sav: Option<&Save>,
        unassigned_color: Option<Rgb<u8>>,
    ) -> Result<(), VicError> {
        let mut savedcolors: HashMap<Rgb<u8>, Rgb<u8>> = HashMap::new();
        let mut statecol: HashMap<String, Rgb<u8>> = HashMap::new();
        let mut tags: HashMap<String, Rgb<u8>> = HashMap::new();
        let mut ids: HashMap<usize, Rgb<u8>> = HashMap::new();

        let mut pathpath = std::path::PathBuf::new();
        pathpath.push("/mnt/c/Steam/steamapps/common/Victoria 3/");

        let province_map = ImageWrap::new(path.to_path_buf(), resize)?;
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
            VicError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "{:?}: failed at color: {:X}{:X}{:X} and coordinate: {}x, {}y",
                    self,
                    color[0],
                    color[1],
                    color[2],
                    i % width,
                    i / width
                ),
            )))
        };

        for (i, color) in new_map.pixels_mut().enumerate().map(|x| (x.0 as u32, x.1)) {
            if let Some(l) = progress_frequency {
                if i % (l * width) == 0 {
                    println!(
                        "{}\t/{}\t{:.0}%",
                        i / width,
                        height,
                        (100.0 * ((i / width) as f64)) / (height as f64)
                    );
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
                                            colore = datacol.scale_to(&defcol, *factor).unravel();
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
                            if let Some(ctag) = sav.unwrap().get_owners(index).map(|(_, c)| c.tag())
                            {
                                if let Some(&newc) = tags.get(ctag) {
                                    draw = newc
                                } else if let Some(newc) = inp.get_country_color_tag(&ctag) {
                                    draw = newc;
                                } else {
                                    draw = *color;
                                }
                                tags.insert(ctag.clone(), draw);
                            } else {
                                draw = unassigned_color.unwrap_or(*color);
                            }
                        } // _ => panic!()
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

        let mid = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_add(0));
        let above = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_add(1));
        let right = inp.pixels_offset(|x| x.checked_add(0), |y| y.checked_sub(1));
        let below = inp.pixels_offset(|x| x.checked_sub(1), |y| y.checked_add(0));
        let left = inp.pixels_offset(|x| x.checked_add(1), |y| y.checked_add(0));

        let mut blank = inp.new_empty(ColorWrap::from(Rgb::from([255, 255, 255])));

        //Zips a mutable iterator over a wite map with:
        //an iterator over neighboring pixels, folding into True if all pixels are equal, and False if they are not.
        {
            let temp = blank.pixels_mut().zip(
                mid.zip(above.zip(right.zip(below.zip(left))))
                    .map(|x| [x.0, x.1 .0, x.1 .1 .0, x.1 .1 .1 .0, x.1 .1 .1 .1])
                    .map(|x| x.iter().filter_map(|&x| x).collect::<HashSet<_>>().len() == 1),
            );

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
