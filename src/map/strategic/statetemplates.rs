
use regex::Regex;
use image::Rgb;

use std::error::Error;


#[derive(Debug, Default)]
pub struct StateTemplate {
    name:           String,
    // naiive id first, as read from game files; followed by an assigned ID as per the logic of the game.
    // in game, state ID is assigned iteratively, independently of the ID in the game files.
    id:             Option<(usize, usize)>,
    // subsistence:    Option<String>,
    provinces:      Option<Vec<Rgb<u8>>>,
    offset:         Option<usize>,
    // arable_land:    Option<u16>,
    // resources:      Option<Vec<(String, u16)>>,
    ocean:          bool,
}

impl StateTemplate {
    pub fn new(data: &mut impl Iterator<Item = String>, ocean: bool) -> Result<Option<Self>, Box<dyn Error>> {

        let mut ret = Self::default();
        ret.ocean = ocean;

        let id_reg = Regex::new(
            // Example text
            /*
            (STATE_LOWER_EGYPT)[1] = {
                id = (175)[2]
                subsistence_building = "(building_subsistence_farms)[3]"
                provinces = { ("x2050A0" "x409060" "x48E2A5" "x5011E0" "x50D0E0" "x6F13AD" "x7ABE4B" "xA0D020" "xC09060" "xC37990" "xD050E0" "xF47AAA")[4] }
                traits = { "state_trait_nile_river" }
                city = "xC09060"
                port = "xA0D020"
                farm = "x409060"
                mine = "xD050E0"
                wood = "x5011E0"
                arable_land = (120)[5]
                arable_resources = { ("bg_wheat_farms" "bg_livestock_ranches" "bg_cotton_plantations" "bg_opium_plantations" "bg_tobacco_plantations" "bg_sugar_plantations" "bg_banana_plantations")[6] }
                capped_resources = {
                (    bg_sulfur_mining = 20
                    bg_logging = 2
                    bg_fishing = 6)[7]
                }
                naval_exit_id = 3034
            (})[0]
            */
            r#"^([A-Z_0-9]+) = \{|^[\t\s]+id = ([0-9]+)|^[\t\s]+subsistence_building = "([A-Za-z_]+)"|^[\t\s]+provinces = \{ (["0-9x\sA-F]+) \}|^[\t\s]+arable_land = ([0-9]+)|^[\t\s]+arable_resources = \{ (["\sa-zA-Z_]+) \}|^[\t\s]+capped_resources = \{\n(["\sa-zA-Z_=0-9]+)\n[\t\s]+\}|^\}"#
        ).unwrap();

        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                // println!("{:?}", b);
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(Some(ret))
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    ret.name = c.to_owned();
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    ret.id = Some((0, c.parse()?));
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(m.as_str())) {
                    ret.provinces = Some(c.split(' ').map(|x| Rgb::from([u8::from_str_radix(&x[2..4], 16).unwrap(), u8::from_str_radix(&x[4..6], 16).unwrap(), u8::from_str_radix(&x[6..8], 16).unwrap()])).collect());
                }
            }
        }
        Ok(None)
    }
    pub fn size(&self) -> usize {
        if let Some(a) = &self.provinces {
            a.len()
        } else {
            0
        }
    }
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }
    pub fn offset(&self) -> Result<usize, Box<dyn Error>> {
        if let Some(off) = self.offset {
            Ok(off)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "t")))
        }
    }
    /// checks if state contains province with ID (color).
    pub fn contains(&self, color: Rgb<u8>) -> bool {
        if let Some(provinces) = &self.provinces {
            for &province in provinces {
                if province == color {
                    return true
                }
            }
        }
        false
    }
    pub fn is_ocean(&self) -> bool {
        self.ocean
    }
    pub fn set_id(&mut self, id: usize) {
        if let Some(a) = &mut self.id {
            self.id = Some((id, a.1))
        } else {
            self.id = Some((id, 0))
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> (usize, usize) {
        self.id.unwrap()
    }
}