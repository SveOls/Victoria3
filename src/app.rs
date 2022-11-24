use super::data::Info;


use fltk::{app, prelude::*, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};


pub fn run() {

    println!("{:?}", dirs::cache_dir());
    println!("{:?}", dirs::config_dir());
    println!("{:?}", dirs::data_dir());
    println!("{:?}", dirs::data_local_dir());
    println!("{:?}", dirs::desktop_dir());
    println!("{:?}", dirs::document_dir());
    println!("{:?}", dirs::download_dir());
    println!("{:?}", dirs::executable_dir());
    println!("{:?}", dirs::font_dir());
    println!("{:?}", dirs::home_dir());
    println!("{:?}", dirs::picture_dir());
    println!("{:?}", dirs::preference_dir());
    println!("{:?}", dirs::public_dir());
    println!("{:?}", dirs::runtime_dir());
    println!("{:?}", dirs::state_dir());
    println!("{:?}", dirs::template_dir());
    println!("{:?}", dirs::video_dir());
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    let mut t = true;
    but.set_callback(move |_| {
        if t {
            frame.set_label("Hello world");
            frame.show();
        } else {
            frame.hide()
        }
        t = !t
    });

    app.run().unwrap();
}

