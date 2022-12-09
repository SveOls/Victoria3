use std::collections::{HashMap, HashSet};
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
    premade_lines: HashMap<(usize, Coloring), ImageWrap>,
    color_map: Coloring,
    premade_color: HashMap<(usize, Coloring), ImageWrap>,
    /// color of ocean
    sea_color: Option<ColorWrap>,
    /// color of data.
    data_color: Option<ColorWrap>,
    provice_map_path: Option<PathBuf>,
    default_color: ColorWrap,
    sea_province_borders: bool,
    scale_fn: Option<(fn(f64, f64) -> f64, f64)>,
    /// memoization of province color to its color and owner. Separated options are for cases where
    /// the color is erased, but the other data is still relevant.
    owners: HashMap<(usize, Rgb<u8>), Option<([Option<Rgb<u8>>; 3], Option<[usize; 2]>)>>,
}

impl MapDrawer {
    pub fn resize(&mut self, inp: f64) {
        self.resize = Some(inp)
    }
    pub fn sea_province_borders(&mut self, inp: bool) {
        self.sea_province_borders = inp
    }
    /// generally a religion, culture, or something else.
    pub fn set_numerator(&mut self, inp: Option<HashMap<usize, usize>>) {
        self.numerator = inp
    }
    /// generally per capita or per area
    pub fn set_denominator(&mut self, inp: Option<HashMap<usize, usize>>) {
        self.denominator = inp
    }
    pub fn set_sea_color(&mut self, inp: Option<ColorWrap>) {
        self.sea_color = inp
    }
    pub fn set_data_scale(&mut self, inp: Option<(fn(f64, f64) -> f64, f64)>) {
        self.scale_fn = inp
    }
    pub fn set_color(&mut self, inp: Option<ColorWrap>) {
        self.data_color = inp
    }
    pub fn darkmode(&mut self, inp: ColorWrap) {
        self.default_color = inp
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
    pub fn clear(&mut self) {
        *self = Self::default()
    }
    pub fn extremify(&mut self, stretch: bool) {
        if self.data_color.is_none() {
            self.data_color = Some(self.default_color.inverse())
        } else if stretch {
            let temp = self.default_color;
            self.data_color = self.data_color.map(|x| x.stretch(&temp));
        }
    }

    pub fn draw(
        &mut self,
        info: &Info,
        save_id: usize,
        save_to: PathBuf,
        map_data: bool,
    ) -> Result<(), VicError> {
        let mut temp = Coloring::StateTemplates;
        std::mem::swap(&mut temp, &mut self.color_map);
        self.draw_map_coloring(info, save_id)?;
        std::mem::swap(&mut temp, &mut self.color_map);

        self.draw_map_coloring(info, save_id)?;
        self.draw_lines_coloring(info, save_id)?;

        let mut saver = self
            .premade_color
            .get(&(save_id, self.color_map))
            .unwrap()
            .clone();

        if let Some(sea_color) = self.sea_color {
            for pix in saver.pixels_mut() {
                if self
                    .owners
                    .get(&(save_id, *pix))
                    .map_or(false, |x| x.is_none())
                {
                    *pix = sea_color.unravel();
                }
            }
        }

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

            if let Some((f, a)) = self.scale_fn {
                data.values_mut().for_each(|x| *x = f(*x, a));
            }

            let max = data.values().fold(0.0f64, |a, &b| a.max(b));
            let min = data.values().fold(1.0f64, |a, &b| a.min(b));

            data.values_mut()
                .for_each(|x| *x = (*x - min) / (max - min));

            let map = info.get_map()?;
            let save = info.get_save(save_id)?;

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
                .zip(
                    self.premade_lines
                        .get(&(save_id, self.lines))
                        .unwrap()
                        .pixels(),
                )
                .filter(|&(_, &y)| {
                    (y == Rgb::from([0, 0, 0]))
                        || (y == Rgb::from([127, 127, 127]) && self.sea_province_borders)
                })
                .for_each(|(x, _)| *x = Rgb::from([0, 0, 0]))
        }

        crate::utilities::save3(save_to, &saver)?;

        Ok(())
    }
    fn draw_lines_coloring(&mut self, info: &Info, save_id: usize) -> Result<(), VicError> {
        let base_map = match self.premade_color.get(&(save_id, self.lines)) {
            Some(a) => a,
            None => {
                std::mem::swap(&mut self.lines, &mut self.color_map);
                self.draw_map_coloring(info, save_id)?;
                std::mem::swap(&mut self.lines, &mut self.color_map);
                self.premade_color.get(&(save_id, self.lines)).unwrap()
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
                .map(|x| {
                    (
                        x.iter().filter_map(|&x| x).fold(true, |a, b| {
                            a && self
                                .owners
                                .get(&(save_id, *b))
                                .map(|y| y.is_none())
                                .unwrap_or(false)
                        }),
                        x,
                    )
                })
                .map(|(a, x)| {
                    (
                        a,
                        x.iter().filter_map(|&x| x).collect::<HashSet<_>>().len() == 1,
                    )
                });

        let mut blank = base_map.new_empty(ColorWrap::from(Rgb::from([0xFF, 0xFF, 0xFF])));

        let temp = blank.pixels_mut().zip(cross);

        for (pixel, (ocean, condition)) in temp {
            if !condition {
                if ocean {
                    *pixel = Rgb::from([0x7F, 0x7F, 0x7F])
                } else {
                    *pixel = Rgb::from([0x00, 0x00, 0x00])
                }
            }
        }

        self.premade_lines.insert((save_id, self.lines), blank);

        Ok(())
    }
    fn draw_map_coloring(&mut self, info: &Info, save_id: usize) -> Result<(), VicError> {
        if self.premade_color.contains_key(&(save_id, self.color_map)) {
            return Ok(());
        }
        let province_map = self
            .premade_color
            .remove(&(save_id, Coloring::Provinces))
            .unwrap_or(ImageWrap::new(self.get_path()?, self.resize)?);

        if self.color_map == Coloring::None {
            let new_map = province_map.new_empty(ColorWrap::from(Rgb::from([0xFF, 0xFF, 0xFF])));
            self.premade_color
                .insert((save_id, Coloring::Provinces), province_map);
            self.premade_color
                .insert((save_id, self.color_map), new_map);
            return Ok(());
        }

        let mut new_map = province_map.clone();

        let mut memo = HashMap::new();

        let map = info.get_map()?;
        let save = info.get_save(save_id)?;

        for color in new_map.pixels_mut() {
            let updatedcolor;
            match self.owner_getter(color, save_id)? {
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
                            self.owners
                                .insert((save_id, *color), Some(([None; 3], temp_owners)));
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
                        self.owner_inserter(color, updatedcolor, save_id);
                    } else {
                        updatedcolor = *color;
                        self.owners.insert((save_id, *color), None);
                    }
                }
                None => updatedcolor = *color,
            }
            *color = updatedcolor;
        }
        self.premade_color
            .insert((save_id, Coloring::Provinces), province_map);
        self.premade_color
            .insert((save_id, self.color_map), new_map);
        Ok(())
    }

    fn owner_getter(
        &self,
        color: &Rgb<u8>,
        save_id: usize,
    ) -> Result<Option<(Option<Rgb<u8>>, Option<usize>)>, VicError> {
        match self.color_map {
            Coloring::StateTemplates => Ok(self
                .owners
                .get(&(save_id, *color))
                .map(|i| i.map(|(x, y)| (x[0], y.map(|z| z[0]))))
                .unwrap_or(Some((None, None)))),
            Coloring::SaveStates => Ok(self
                .owners
                .get(&(save_id, *color))
                .map(|i| i.map(|(x, y)| (x[1], y.map(|z| z[0]))))
                .unwrap_or(Some((None, None)))),
            Coloring::SaveCountries => Ok(self
                .owners
                .get(&(save_id, *color))
                .map(|i| i.map(|(x, y)| (x[2], y.map(|z| z[1]))))
                .unwrap_or(Some((None, None)))),
            Coloring::Provinces => Ok(Some((Some(*color), None))),
            Coloring::None => Err(VicError::temp()),
        }
    }
    fn owner_inserter(&mut self, color: &Rgb<u8>, updatedcolor: Rgb<u8>, save_id: usize) {
        match self.color_map {
            Coloring::StateTemplates => self
                .owners
                .entry((save_id, *color))
                .and_modify(|a| a.iter_mut().for_each(|x| x.0[0] = Some(updatedcolor))),
            Coloring::SaveStates => self
                .owners
                .entry((save_id, *color))
                .and_modify(|a| a.iter_mut().for_each(|x| x.0[1] = Some(updatedcolor))),
            Coloring::SaveCountries => self
                .owners
                .entry((save_id, *color))
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
