#![allow(unused_imports)]

use crate::data::{DataTypes, Info};
use crate::draw::{self, Coloring, MapDrawer};
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

use self::scan_tab::ScanTab;

mod draw_tab;
mod scan_tab;

pub fn run() -> Result<(), VicError> {
    // println!("{:?}", dirs::cache_dir());

    let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut info = Info::new();
    let mut mapdrawer = MapDrawer::default();
    // let mut game_dir: PathBuf;
    // let mut save_dir: PathBuf;
    // let mut map = None;

    let (s, r) = app::channel::<usize>();

    let mut wind = Window::default().with_size(1280, 720);

    let tab_edge_buffer = 5; // buffer between edge of window and tab box
    let tab = Tabs::default()
        .with_pos(wind.x() + tab_edge_buffer, wind.y() + tab_edge_buffer)
        .with_size(
            wind.w() - 2 * tab_edge_buffer,
            wind.h() - 2 * tab_edge_buffer,
        );

    let tab_box_height = 25;
    let mut scan_tab = scan_tab::ScanTab::new(&tab, s, tab_box_height);
    let mut draw_tab = draw_tab::DrawTab::new(&tab, s, tab_box_height);

    tab.end();

    wind.end();
    wind.show();

    while app.wait() {
        match r.recv() {
            Some(100) => (_, info, mapdrawer) = draw_tab.draw(info, mapdrawer, &mut app)?,
            Some(101) => {
                (_, info, mapdrawer) = draw_tab.quick_draw_countries(info, mapdrawer, &mut app)?
            }
            Some(102) => {
                (_, info, mapdrawer) = draw_tab.quick_draw_states(info, mapdrawer, &mut app)?
            }
            Some(103) => draw_tab.lock(),
            Some(104) => draw_tab.check_custom_color(false), // from custom_color textbox callback
            Some(105) => draw_tab.check_custom_color(true),
            Some(106) => draw_tab.check_default_color(false), // from custom_default textbox callback
            Some(107) => draw_tab.check_default_color(true),
            Some(108) => draw_tab.check_custom_watercolor(false), // from custom_default textbox callback
            Some(109) => draw_tab.check_custom_watercolor(true),
            Some(200) => {
                info.clear(DataTypes::Save);
                (info, _) = match info.find_path(DataTypes::Save, &mut app, &mut scan_tab)? {
                    (Some(err), returned_info, new_save_path) => {
                        println!("{:?}", err);
                        (returned_info, new_save_path)
                    }
                    (None, returned_info, new_save_path) => (returned_info, new_save_path),
                }
            }
            Some(201) => {
                info.clear(DataTypes::Map);
                (info, _) = match info.find_path(DataTypes::Map, &mut app, &mut scan_tab)? {
                    (Some(err), returned_info, new_map_path) => {
                        println!("{:?}", err);
                        (returned_info, new_map_path)
                    }
                    (None, returned_info, new_map_path) => {
                        mapdrawer.set_path(new_map_path.clone());
                        (returned_info, new_map_path)
                    }
                }
            }
            _ => {}
        }
    }

    // app.run()?;
    Ok(())
}

fn read_only_checkbutton(inp: &mut CheckButton) {
    inp.set_value(!inp.value())
}

impl Info {
    /// in vicapp/mod.rs, not in the normal impl location. might move, or turn into function
    pub fn find_path(
        mut self,
        load_type: DataTypes,
        app: &mut app::App,
        scan_tab: &mut ScanTab,
    ) -> Result<(Option<VicError>, Self, PathBuf), VicError> {
        let mut dialog;
        let win;
        let lin;
        match load_type {
            DataTypes::Map => {
                dialog = NativeFileChooser::new(NativeFileChooserType::BrowseDir);
                win = Path::new("c:/Steam/steamapps/common/Victoria 3").to_path_buf();
                lin = Path::new("/mnt/c/Steam/steamapps/common/Victoria 3").to_path_buf();
            }
            DataTypes::Save => {
                dialog = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
                win =
                    Path::new("c:/Users/sverr/Documents/Paradox Interactive/Victoria 3/save games")
                        .to_path_buf();
                lin = Path::new(
                    "/mnt/c/Users/sverr/Documents/Paradox Interactive/Victoria 3/save games",
                )
                .to_path_buf();
            }
        };
        // temp code for convenience
        if let Some(p) = scan_tab.path(load_type) {
            dialog.set_directory(&p).unwrap();
        } else if lin.is_dir() {
            dialog.set_directory(&lin.as_path()).unwrap();
        } else if win.is_dir() {
            dialog.set_directory(&win.as_path()).unwrap();
        }

        dialog.show();

        let (sa, ra) = app::channel::<(Option<VicError>, Info)>();

        thread::spawn({
            let t = dialog.filename();
            move || {
                sa.send((self.load(t.as_path(), load_type).err(), self));
            }
        });

        'outer: {
            while app.wait() {
                if let Some(c) = ra.recv() {
                    scan_tab.update(load_type, dialog.filename(), c.0.is_none());
                    break 'outer Ok((c.0, c.1, dialog.filename()));
                }
            }
            Err(VicError::temp())
        }
    }
}
