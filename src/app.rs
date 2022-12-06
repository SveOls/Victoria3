#![allow(unused_imports)]

use crate::data::Info;
use crate::draw::{Coloring, MapDrawer};
use crate::error::VicError;
use crate::wrappers::ColorWrap;

use std::path::{Path, PathBuf};

use fltk::button::{CheckButton, ToggleButton};
use fltk::dialog::{FileChooser, FileChooserType};
use fltk::enums::CallbackTrigger;
use fltk::group::Scroll;
use fltk::menu;
use fltk::{app, prelude::*, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};
use fltk::{enums, input};

pub fn run() -> Result<(), VicError> {
    // println!("{:?}", dirs::cache_dir());
    // println!("{:?}", dirs::config_dir());
    // println!("{:?}", dirs::data_dir());
    // println!("{:?}", dirs::data_local_dir());
    // println!("{:?}", dirs::desktop_dir());
    // println!("{:?}", dirs::document_dir());
    // println!("{:?}", dirs::download_dir());
    // println!("{:?}", dirs::executable_dir());
    // println!("{:?}", dirs::font_dir());
    // println!("{:?}", dirs::home_dir());
    // println!("{:?}", dirs::picture_dir());
    // println!("{:?}", dirs::preference_dir());
    // println!("{:?}", dirs::public_dir());
    // println!("{:?}", dirs::runtime_dir());
    // println!("{:?}", dirs::state_dir());
    // println!("{:?}", dirs::template_dir());
    // println!("{:?}", dirs::video_dir());
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut info = Info::new();
    let mut mapdrawer = MapDrawer::default();
    let mut game_dir = PathBuf::from("/mnt/c/Steam/steamapps/common/Victoria 3");
    let mut save_dir =
        PathBuf::new().join("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3");

    // println!("{:?}", game_dir);
    // if let Some(a) = find_dir("game")? {
    //     game_dir = PathBuf::new().join(&a)
    // }
    // println!("{:?}", game_dir);
    // println!("{:?}", save_dir);
    // let save_files = find_files("save")?;
    // println!("{:?}", save_files);
    // for i in save_files {
    //     info.load_save(Path::new(&i))?;
    // }
    // println!("{:?}", save_dir);

    let app = app::App::default();

    let mut wind = Window::default().with_size(720, 480);

    // let frame = Frame::default().with_size(200, 100).center_of(&wind);
    // let mut but = Button::new(360, 20, 100, 40, "Scan Map");
    // let mut tub = input::Input::new(580, 420, 120, 40, "Culture");
    // tub.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut btu = input::Input::new(580, 360, 120, 40, "Name");
    // btu.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut tub = Button::new(560, 420, 160, 40, "Draw");
    let mut ttt = Button::new(560, 20, 160, 40, "(fast) Draw States");
    let mut eee = Button::new(560, 80, 160, 40, "(fast) Draw Countries");
    let mut ubt = Button::new(20, 80, 100, 40, "Load Save");
    let mut utb = Button::new(20, 20, 100, 40, "Install Location");

    let mut light_mode = CheckButton::new(380, 360, 100, 40, "light mode");
    let mut boop = CheckButton::new(380, 420, 100, 40, "data");

    let mut choiceden = menu::Choice::new(550, 145, 150, 30, "Select denom");
    let mut choicenum = menu::Choice::new(550, 200, 150, 30, "Select numer");
    let mut choicecolor = menu::Choice::new(550, 255, 150, 30, "Select color");
    let mut choicelines = menu::Choice::new(550, 310, 150, 30, "Select lines");
    choicenum.add_choice(" religion| culture| population");
    choiceden.add_choice(" None| population| area");
    choicecolor.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
    choicelines.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");

    choiceden.set_value(0);
    choicecolor.set_value(3);
    choicelines.set_value(3);
    choicenum.set_value(0);

    wind.end();
    wind.show();

    let (s, r) = app::channel::<usize>();

    // but.set_callback(move |_| s.send(1));
    // tub.set_callback(move |_| s.send(2));
    ubt.set_callback(move |_| s.send(3));
    utb.set_callback(move |_| s.send(4));
    eee.emit(s, 5);
    ttt.emit(s, 6);

    // tub.emit(s, 2);
    tub.emit(s, 0);

    wind.end();
    wind.show();

    while app.wait() {
        match r.recv() {
            Some(0) => {
                // temp fix
                if boop.value() {
                    choicecolor.set_value(3);
                }
                //
                let (num, col) = match choicenum.value() {
                    0 => info.religion(&btu.value())?,
                    1 => info.culture(&btu.value())?,
                    2 if light_mode.value()  => (info.population()?, Some(ColorWrap::from(image::Rgb::from([0x00,0x00,0x00])))),
                    2 if !light_mode.value() => (info.population()?, Some(ColorWrap::from(image::Rgb::from([0xFF,0xFF,0xFF])))),
                    _ => return Err(VicError::temp()),
                };
                mapdrawer.set_numerator(Some(num));
                mapdrawer.set_color(col);
                match choicecolor.value() {
                    0 => mapdrawer.set_color_map(Coloring::None),
                    1 => mapdrawer.set_color_map(Coloring::Provinces),
                    2 => mapdrawer.set_color_map(Coloring::StateTemplates),
                    3 => mapdrawer.set_color_map(Coloring::SaveStates),
                    4 => mapdrawer.set_color_map(Coloring::SaveCountries),
                    _ => {}
                }
                match choicelines.value() {
                    0 => mapdrawer.set_lines(Coloring::None),
                    1 => mapdrawer.set_lines(Coloring::Provinces),
                    2 => mapdrawer.set_lines(Coloring::StateTemplates),
                    3 => mapdrawer.set_lines(Coloring::SaveStates),
                    4 => mapdrawer.set_lines(Coloring::SaveCountries),
                    _ => {}
                }
                match choiceden.value() {
                    0 => mapdrawer.set_denominator(None),
                    1 => mapdrawer.set_denominator(Some(info.population()?)),
                    2 => mapdrawer.set_denominator(Some(info.area()?)),
                    _ => {}
                }
                mapdrawer.darkmode(!light_mode.value());
                mapdrawer.set_sea_color(ColorWrap::from(image::Rgb::from([0, 100, 200])));


                mapdrawer.draw(&info, PathBuf::from("output"), boop.value())?;

                println!("draw complete");
            }
            Some(1) => {
                // println!("{game_dir:?}");
                // mapdrawer.set_path(game_dir.clone());
                // info.load_map(&game_dir)?;
                // println!("game loaded");
            }
            Some(2) => {
                // info.test(
                //     &game_dir,
                //     Some(tub.value().to_owned()),
                //     None,
                //     false,
                //     false,
                //     beit.value(),
                // )?;
                // println!("cul test complete");
            }
            Some(3) => {
                info.clear_saves();
                let save_files = find_files("save")?;
                println!("{:?}", save_files);
                // choice.clear();
                for i in save_files {
                    let a = Path::new(&i);
                    // choice.add("test", enums::Shortcut::None, menu::MenuFlag::Normal, |_| {});
                    info.load_save(Path::new(a))?;
                }
            }
            Some(4) => {
                if let Some(a) = find_dir("game")? {
                    game_dir = PathBuf::from(a);
                    println!("{game_dir:?}");
                    mapdrawer.set_path(game_dir.clone());
                    info.load_map(&game_dir)?;
                    println!("game loaded");
                }
            }
            Some(5) => {
                choicecolor.set_value(4);
                boop.set_value(false);
                s.send(0);
            }
            Some(6) => {
                choicecolor.set_value(3);
                boop.set_value(false);
                s.send(0);
            }
            _ => {}
        }
    }

    // app.run()?;
    Ok(())
}

pub fn find_dir(inp: &str) -> Result<Option<String>, VicError> {
    let mut chooser = FileChooser::new(
        ".",                        // directory
        "",                         // filter or pattern
        FileChooserType::Directory, // chooser type
        inp,                        // title
    );
    chooser.set_preview(false);
    chooser.set_size(480, 480);

    chooser.show();

    chooser.window();

    while chooser.shown() {
        app::wait();
    }

    Ok(chooser.directory())
}

pub fn find_files(inp: &str) -> Result<Vec<String>, VicError> {
    let mut chooser = FileChooser::new(
        ".",                     // directory
        "*",                     // filter or pattern
        FileChooserType::Single, // chooser type
        inp,                     // title
    );
    chooser.set_preview(false);
    chooser.set_size(480, 480);

    chooser.show();

    chooser.window();

    while chooser.shown() {
        app::wait();
    }

    Ok((0..chooser.count())
        .map_while(|i| chooser.value(i))
        .collect())
}
