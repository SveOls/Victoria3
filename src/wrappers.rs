

// use std::error::Error;
use std::{path::{PathBuf, Path}, ops::Index};
use crate::error::VicError;
// use fltk::enums::Color;
use image::{ImageBuffer, Rgb};

pub fn get_provinces(flip: bool, shrink: Option<f64>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, VicError> {
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


#[derive(Clone)]
pub struct ImageWrap(ImageBuffer<Rgb<u8>, Vec<u8>>);

impl ImageWrap {
    pub fn new(inp: PathBuf, shrink: Option<f64>) -> Result<Self, VicError> {
        if let Some(a) = shrink {
            let tempimg = image::open(inp.join("game/map_data/provinces.png"))?;
            Ok(Self(tempimg.resize(((a * tempimg.width() as f64) / 100.0) as u32, u32::MAX, image::imageops::FilterType::Nearest).into_rgb8()))
        } else {
            Ok(Self(image::open(inp.join("game/map_data/provinces.png"))?.into_rgb8()))
        }
    }
    pub fn new_empty(&self, pix: ColorWrap) -> Self {
        let (w, h) = self.0.dimensions();
        Self(ImageBuffer::from_pixel(w, h, pix.unravel()))
    }
    pub fn pixels(&self) -> impl Iterator<Item = &Rgb<u8>> {
        self.0.pixels()
    }
    pub fn vflip_pixels(&self) -> impl Iterator<Item = &Rgb<u8>> {
        self.0.rows().rev().flatten()
    }
    pub fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Rgb<u8>> {
        self.0.pixels_mut()
    }
    pub fn vflip_pixels_mut(&mut self) -> impl Iterator<Item = &mut Rgb<u8>> {
        self.0.rows_mut().rev().flatten()
    }
    pub fn pixels_offset(&self, dx: fn(u32) -> Option<u32>, dy: fn(u32) -> Option<u32>) -> impl Iterator<Item = Option<&Rgb<u8>>> {
        self.0.enumerate_pixels()
            .map(move |(x, y, _)| (dx(x), dy(y)))
            .map(|(x, y)| x.and_then(|x1| y.and_then(|y1| self.0.get_pixel_checked(x1, y1))))
    }
    pub fn save(&self, to: &Path) -> Result<(), VicError> {
        match self.0.save(to) {
            Ok(_) => Ok(()),
            Err(e) => Err(VicError::Other(Box::new(e)))
        }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColorWrap {
    Rgb(Rgb<u8>),
}

impl Index<usize> for ColorWrap {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            ColorWrap::Rgb(a) => &a[index],
        }
    }
}

impl ColorWrap {
    pub fn from(inp: Rgb<u8>) -> Self {
        Self::Rgb(inp)
    }
    pub fn unravel(self) -> Rgb<u8> {
        match self {
            ColorWrap::Rgb(a) => a
        }
    }
    pub fn scale_to(&self, to: &Self, scale: f64) -> Self {
        match (self, to) {
            (ColorWrap::Rgb(_), ColorWrap::Rgb(_)) => {
                ColorWrap::Rgb(Rgb::from([self.scale_at(to, scale, 0), self.scale_at(to, scale, 1), self.scale_at(to, scale, 2)]))
            }
        }
    }
    /// accepts "format{num num num}"
    ///
    /// whitespace acceptable anywhere (except between num)
    ///
    /// also accepts "num num num"
    ///
    /// also accepts strings where the first 6 hex chars are the desired output,
    /// and where a split on whitespace, '{' or '}' followed by filtering
    /// empty items gets collected into a single-value array.
    pub fn to_colorwrap(inp: &str) -> Result<Self, VicError> {
        match inp.split(|c: char| c == '{' || c == '}' || c.is_whitespace()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().as_slice() {

            ["hsv", a, b, c] => Self::from_hsv(360.0 * a.parse::<f64>()?, b.parse()?, c.parse()?),
            ["hsv360", a, b, c] => Self::from_hsv(a.parse::<f64>()?, b.parse::<f64>()? * 0.01, c.parse::<f64>()? * 0.01),
            ["rgb", a, b, c] | [a, b, c] => Self::from_rgb([a.parse()?, b.parse()?, c.parse()?]),

            // later: skip any number before "x" to avoid "0x123456" becoming "01 23 45" instead of "12 34 56"
            [a]         => Self::from_hex(a.chars().filter_map(|c| c.to_digit(16).map(|d| d as u8))),

            _           => Err(VicError::SaveError)
        }
    }
    fn scale_at(&self, to: &Self, scale: f64, at: usize) -> u8 {
        (self[at] as f64 *scale + to[at] as f64 *(1.0 - scale)) as u8
    }
    fn from_hsv(h: f64, s: f64, v: f64) -> Result<Self, VicError> {
        Ok(Self::Rgb(Rgb::from([
            Self::hsv_id(0, s*v, v - s*v, h),
            Self::hsv_id(1, s*v, v - s*v, h),
            Self::hsv_id(2, s*v, v - s*v, h)
        ])))
    }
    fn hsv_id(id: usize, c: f64, m: f64, h: f64) -> u8 {
        if id == ((1 +   (h as usize / 60)) / 2) % 3 {
            ((c + m)*255.0).round() as u8
        } else if id == (1 + 2* (h as usize / 60)) % 3 {
            ((c*(1.0 - ((((h as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0).round() as u8
        } else if id == ((4 +   (h as usize / 60)) / 2) % 3 {
            ((m)*255.0).round() as u8
        } else {
            panic!()
        }
    }
    fn from_rgb(inp: [f64; 3]) -> Result<Self, VicError> {
        if inp[0] + inp[1] + inp[2] > 4.0 {
            Ok(Self::Rgb(Rgb::from([inp[0] as u8, inp[1] as u8, inp[2] as u8])))
        } else {
            Self::from_rgb([inp[0] * 255.0, inp[1] * 255.0, inp[2] * 255.0])
        }
    }
    fn from_hex(mut inp: impl Iterator<Item = u8>) -> Result<Self, VicError> {
        Ok(Self::Rgb(Rgb::from(
            [Self::fold_to_int(&mut inp)?, Self::fold_to_int(&mut inp)?, Self::fold_to_int(&mut inp)?]
        )))
    }
    fn fold_to_int(inp: &mut impl Iterator<Item = u8>) -> Result<u8, VicError> {
        inp.next().and_then(|x| inp.next().map(|y| 0x10 * x + y))
            .ok_or(VicError::SaveError)
    }
}


