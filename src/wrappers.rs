

// use std::error::Error;
use std::path::{PathBuf, Path};
use crate::error::VicError;
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
    pub fn new_empty(&self, pix: RgbWrap) -> Self {
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
pub struct RgbWrap(Rgb<u8>);

impl RgbWrap {
    pub fn from(inp: Rgb<u8>) -> Self {
        Self(inp)
    }
    pub fn unravel(self) -> Rgb<u8> {
        self.0
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
    pub fn to_rgb(inp: &str) -> Result<Self, VicError> {

        let data = inp.split(|c: char| c == '{' || c == '}' || c.is_whitespace()).filter(|s| !s.is_empty()).collect::<Vec<&str>>();

        let mut temp = [0, 0, 0];

        match data.as_slice() {
            ["hsv", a, b, c] | ["hsv360", a, b, c] => {

                let mut hsvdata = [360.0, 100.0, 100.0];

                match data[0] {
                    "hsv" => {
                        hsvdata[0] *= a.parse::<f64>()?;
                        hsvdata[1] *= b.parse::<f64>()?;
                        hsvdata[2] *= c.parse::<f64>()?;
                    }
                    "hsv360" => {
                        hsvdata[0]  = a.parse::<f64>()?;
                        hsvdata[1]  = b.parse::<f64>()?;
                        hsvdata[2]  = c.parse::<f64>()?;
                    }
                    _ => panic!()
                }

                let c = ((hsvdata[1]*hsvdata[2]) as f64) / 10000.0;
                let m = ( hsvdata[2]             as f64  / 100.0) - c;

                temp[((1 +   (hsvdata[0] as usize / 60)) / 2) % 3] =
                    ((c + m)*255.0)
                    .round().clamp(0.0, 255.0) as u8;

                temp[(1 + 2* (hsvdata[0] as usize / 60)) % 3     ] =
                    ((c*(1.0 - ((((hsvdata[0] as f64) / 60.0)%2.0) - 1.0).abs()) + m)*255.0)
                    .round().clamp(0.0, 255.0) as u8;

                temp[((4 +   (hsvdata[0] as usize / 60)) / 2) % 3] =
                    ((m)*255.0)
                    .round().clamp(0.0, 255.0) as u8;
            }
            ["rgb", a, b, c] | [a, b, c] => {
                let rgbdata = [a.parse::<f64>()?, b.parse::<f64>()?, c.parse::<f64>()?];

                if rgbdata[0] + rgbdata[1] + rgbdata[2] < 4.0 {
                    temp[0] = (rgbdata[0] * 255.0) as u8;
                    temp[1] = (rgbdata[1] * 255.0) as u8;
                    temp[2] = (rgbdata[2] * 255.0) as u8;
                } else {
                    temp[0] =  a.parse()?;
                    temp[1] =  b.parse()?;
                    temp[2] =  c.parse()?;
                }
            }
            [a] => {
                let b = a.chars().filter_map(|c| c.to_digit(16).and_then(|d| Some(d as u8))).collect::<Vec<u8>>();
                if b.len() >= 6 {
                    temp = [0x10 * b[0] + b[1], 0x10 * b[2] + b[3], 0x10 * b[4] + b[5]];
                } else {
                    panic!()
                }
            }
            _ => panic!()
        }
        Ok(Self(Rgb::from(temp)))
    }

}


