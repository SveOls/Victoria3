#![allow(unused_imports)]

use crate::error::VicError;

use super::data::Info;

use std::path::{Path, PathBuf};

use fltk::dialog::{FileChooser, FileChooserType};
use fltk::{app, prelude::*, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};
use fltk::input;
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
    let mut game_dir = PathBuf::new().join("/mnt/c/Steam/steamapps/common/Victoria 3");
    let mut save_dir = PathBuf::new().join("/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3");

    // println!("{:?}", game_dir);
    // if let Some(a) = find_dir("game")? {
    //     game_dir = PathBuf::new().join(&a)
    // }
    // println!("{:?}", game_dir);
    // println!("{:?}", save_dir);
    let save_files = find_files("save")?;
    println!("{:?}", save_files);
    for i in save_files {
        info.load_save(Path::new(&i))?;
    }
    // println!("{:?}", save_dir);

    let app = app::App::default();
    let mut wind = Window::default().with_size(720, 480);

    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Scan Map");
    wind.end();
    wind.show();

    let (s, r) = app::channel::<()>();

    but.set_callback(move |_| s.send(()));
    wind.end();
    wind.show();

    while app.wait() {
        if r.recv().is_some() {
            info.load_map(&game_dir)?;
            info.test()?;
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
        FileChooserType::Multi, // chooser type
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