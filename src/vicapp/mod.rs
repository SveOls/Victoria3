#![allow(unused_imports)]

use crate::data::{DataTypes, Info};
use crate::draw::{Coloring, MapDrawer};
use crate::error::VicError;
use crate::wrappers::ColorWrap;

use std::path::{Path, PathBuf};
use std::thread;

use fltk::button::{CheckButton, ToggleButton};
use fltk::dialog::{
    FileChooser, FileChooserType, NativeFileChooser, NativeFileChooserOptions,
    NativeFileChooserType,
};
use fltk::enums::{Align, CallbackTrigger};
use fltk::group::{Group, Pack, Scroll, Tabs};
use fltk::menu;
use fltk::output::Output;
use fltk::{app, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};
use fltk::{enums, input};

mod scan_tab;

pub fn run() -> Result<(), VicError> {
    // println!("{:?}", dirs::cache_dir());

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut info = Info::new();
    let mut mapdrawer = MapDrawer::default();
    // let mut game_dir: PathBuf;
    // let mut save_dir: PathBuf;
    // let mut map = None;

    let (s, r) = app::channel::<usize>();

    let mut wind = Window::default().with_size(720, 480);

    let tab_edge_buffer = 5; // buffer between edge of window and tab box
    let tab = Tabs::default()
        .with_pos(wind.x() + tab_edge_buffer, wind.y() + tab_edge_buffer)
        .with_size(
            wind.w() - 2 * tab_edge_buffer,
            wind.h() - 2 * tab_edge_buffer,
        );

    let mut tab_one = scan_tab::ScanTab::new(&tab, s);

    let draw_group = Group::default()
        .with_pos(tab.x(), tab.y() + 25)
        .with_size(tab.w(), tab.h() - 25)
        .with_label("Tab2\t\t");

    // let frame = Frame::default().with_size(200, 100).center_of(&wind);
    // let mut but = Button::new(360, 20, 100, 40, "Scan Map");
    // let mut tub = input::Input::new(580, 420, 120, 40, "Culture");
    // tub.set_trigger(CallbackTrigger::EnterKeyAlways);
    let btu = input::Input::new(580, 360, 120, 40, "Name");
    // btu.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut tub = Button::new(560, 420, 160, 40, "Draw");
    let mut ttt = Button::new(560, 20, 160, 40, "(fast) Draw States");
    let mut eee = Button::new(560, 80, 160, 40, "(fast) Draw Countries");

    let light_mode = CheckButton::new(380, 360, 100, 40, "light mode");
    let mut boop = CheckButton::new(380, 420, 100, 40, "data");

    let mut choiceden =
        menu::Choice::new(550, 145, 150, 30, "Select denom").with_align(Align::PositionMask);
    let mut choicenum =
        menu::Choice::new(550, 200, 150, 30, "Select numer").with_align(Align::Wrap);
    let mut choicecolor =
        menu::Choice::new(550, 255, 150, 30, "Select color").with_align(Align::LeftBottom);
    let mut choicelines =
        menu::Choice::new(550, 310, 150, 30, "Select lines").with_align(Align::LeftTop);
    choicenum.add_choice(" religion| culture| population");
    choiceden.add_choice(" None| population| area");
    choicecolor.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
    choicelines.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");

    choiceden.set_value(0);
    choicecolor.set_value(3);
    choicelines.set_value(3);
    choicenum.set_value(0);

    draw_group.end();
    //---------------

    tab.end();

    wind.end();
    wind.show();

    // but.set_callback(move |_| s.send(1));
    // tub.set_callback(move |_| s.send(2));
    eee.emit(s, 5);
    ttt.emit(s, 6);
    // chg.emit(s, 1);

    // tub.emit(s, 2);
    tub.emit(s, 0);
    // chg.set_value(true);

    wind.end();
    wind.show();

    while app.wait() {
        match r.recv() {
            Some(0) => {
                // temp fix - it can only map on SaveStates, anyway, so why let the app crash?
                if boop.value() {
                    choicecolor.set_value(3);
                }
                //
                let (num, col) = match choicenum.value() {
                    0 => info.religion(&btu.value())?,
                    1 => info.culture(&btu.value())?,
                    2 if light_mode.value() => (
                        info.population()?,
                        Some(ColorWrap::from(image::Rgb::from([0x00, 0x00, 0x00]))),
                    ),
                    2 if !light_mode.value() => (
                        info.population()?,
                        Some(ColorWrap::from(image::Rgb::from([0xFF, 0xFF, 0xFF]))),
                    ),
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
            Some(1) => {}
            Some(2) => {}
            Some(3) => {
                info.clear_save();

                let (sa, ra) = app::channel::<(Option<VicError>, Info, PathBuf)>();

                // (info, _) = match temp_fn_name(NativeFileChooserType::BrowseFile)

                thread::spawn(|| info.find_path(DataTypes::Save, sa));

                (info, _) = match 'outer: {
                    while app.wait() {
                        if let Some(c) = ra.recv() {
                            break 'outer Ok(c);
                        }
                    }
                    Err(VicError::temp())
                }? {
                    (Some(err), returned_info, new_save_path) => {
                        println!("{:?}", err);
                        tab_one.update_save(new_save_path.to_string_lossy().as_ref(), false);
                        (returned_info, new_save_path)
                    }
                    (None, returned_info, c) => {
                        tab_one.update_save(c.to_string_lossy().as_ref(), false);
                        (returned_info, c)
                    }
                };
            }
            Some(4) => {
                info.clear_map();

                let (sa, ra) = app::channel::<(Option<VicError>, Info, PathBuf)>();

                // thread 1
                thread::spawn(|| info.find_path(DataTypes::Map, sa));
                // thread 2
                (info, _) = match 'outer: {
                    while app.wait() {
                        if let Some(c) = ra.recv() {
                            break 'outer Ok(c);
                        }
                    }
                    Err(VicError::temp())
                }? {
                    // match starts once sa sends message (after which it is terminated). Matches on (sa, ra)'s type
                    (Some(err), returned_info, new_game_path) => {
                        println!("{:?}", err);
                        tab_one.update_game(new_game_path.to_string_lossy().as_ref(), false);
                        (returned_info, new_game_path)
                    }
                    (None, returned_info, new_game_path) => {
                        tab_one.update_game(new_game_path.to_string_lossy().as_ref(), true);
                        (returned_info, new_game_path)
                    }
                };
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

impl Info {
    /// in vicapp/mod.rs, not in the normal impl location. might move.
    pub fn find_path(
        mut self,
        load_type: DataTypes,
        sa: app::Sender<(Option<VicError>, Info, PathBuf)>,
    ) {
        let mut dialog = match load_type {
            DataTypes::Map  => NativeFileChooser::new(dbg!(NativeFileChooserType::BrowseDir)),
            DataTypes::Save => NativeFileChooser::new(dbg!(NativeFileChooserType::BrowseFile)),
        };
        dialog.set_directory(&Path::new("C:")).unwrap();
        println!("{:?}", dialog.directory());
        println!("{:?}", dialog.error_message());
        println!("{:?}", dialog.filename());
        dialog.show();
        sa.send((
            self.load(dialog.filename().as_path(), load_type).err(),
            self,
            dialog.filename(),
        ));
    }
}
