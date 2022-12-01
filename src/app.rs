#![allow(unused_imports)]

use crate::error::VicError;

use super::data::Info;

use std::path::{Path, PathBuf};

use fltk::button::{ToggleButton, CheckButton};
use fltk::dialog::{FileChooser, FileChooserType};
use fltk::enums::CallbackTrigger;
use fltk::group::Scroll;
use fltk::{app, prelude::*, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};
use fltk::{input, enums};
use fltk::menu;

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
    let mut game_dir = PathBuf::from("/mnt/c/Steam/steamapps/common/Victoria 3");
    let mut save_dir = PathBuf::new().join("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3");

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
    let mut but = Button::new(360, 20, 100, 40, "Scan Map");
    let mut tub = input::Input::new(580, 420, 120, 40, "Culture");
    tub.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut btu = input::Input::new(580, 360, 120, 40, "Religion");
    btu.set_trigger(CallbackTrigger::EnterKeyAlways);
    // let mut tub = Button::new(240, 210, 80, 40, "TestCul");
    let mut ttt = Button::new(620, 20, 100, 40, "state Map");
    let mut eee = Button::new(620, 80, 100, 40, "Country Map");
    let mut ubt = Button::new(20, 80, 100, 40, "Load Save");
    let mut utb = Button::new(20, 20, 100, 40, "Install Location");

    let mut beit = CheckButton::new(380, 360, 100, 40, "light mode");

    // let mut choice = menu::Choice::new(600, 200, 100, 40, "Select item");


    wind.end();
    wind.show();

    let (s, r) = app::channel::<usize>();

    but.set_callback(move |_| s.send(1));
    tub.set_callback(move |_| s.send(2));
    ubt.set_callback(move |_| s.send(3));
    utb.set_callback(move |_| s.send(4));
    eee.emit(s, 5);
    ttt.emit(s, 6);


    tub.emit(s, 2);
    btu.emit(s, 0);

    wind.end();
    wind.show();

    while app.wait() {
        match r.recv() {
            Some(0) => {
                info.test(&game_dir, None, Some(btu.value().to_owned()), false, false, beit.value())?;
                println!("rel test complete");
            }
            Some(1) => {
                println!("{game_dir:?}");
                info.load_map(&game_dir)?;
                println!("game loaded");
            }
            Some(2) => {
                info.test(&game_dir, Some(tub.value().to_owned()), None, false, false, beit.value())?;
                println!("cul test complete");
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
                        game_dir = PathBuf::from(a)
                    }
            }
            Some(5) => {
                info.test(&game_dir, None, None, true, false, beit.value())?;
                println!("country test complete");
            }
            Some(6) => {
                info.test(&game_dir, None, None, false, true, beit.value())?;
                println!("state test complete");
            }
            _ => {}
        }
    }


    // app.run()?;
    Ok(())
}

pub fn find_dir(inp: &str) -> Result<Option<String>, VicError> {
    let mut chooser = FileChooser::new(
        ".",                    // directory
        "",                    // filter or pattern
        FileChooserType::Directory, // chooser type
        inp,     // title
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
        ".",                    // directory
        "*",                    // filter or pattern
        FileChooserType::Single, // chooser type
        inp,     // title
    );
    chooser.set_preview(false);
    chooser.set_size(480, 480);

    chooser.show();

    chooser.window();

    while chooser.shown() {
        app::wait();
    }


    Ok((0..chooser.count()).map_while(|i| chooser.value(i)).collect())
}