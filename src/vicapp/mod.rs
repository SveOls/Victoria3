#![allow(unused_imports)]

use crate::data::{DataTypes, Info};
use crate::draw::{self, Coloring, MapDrawer};
use crate::error::VicError;
use crate::wrappers::ColorWrap;

use std::path::{Path, PathBuf};
use std::thread;

use fltk::app::sleep;
use fltk::button::{CheckButton, ToggleButton};
use fltk::dialog::{
    FileChooser, FileChooserType, NativeFileChooser, NativeFileChooserOptions,
    NativeFileChooserType,
};
use fltk::enums::{Align, CallbackTrigger, Color, FrameType};
use fltk::group::{Group, Pack, Scroll, Tabs};
use fltk::image::PngImage;
use fltk::input::{FloatInput, Input};
use fltk::menu::Choice;
use fltk::output::Output;
use fltk::prelude::{ButtonExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt};
use fltk::{
    app::{self, App, Scheme, Sender},
    window::Window,
};
use fltk::{button::Button, frame::Frame, prelude::*};

use self::draw_tab::DrawTab;
use self::preview_tab::PreviewTab;
use self::scan_tab::ScanTab;

mod draw_tab;
mod preview_tab;
mod scan_tab;

struct VicWindow {
    wind: Window,
    scan_tab: ScanTab,
    draw_tab: DrawTab,
    preview_tab: PreviewTab,
    output_info: Output
}

impl VicWindow {
    fn new(s: Sender<usize>) -> Self {
        let wind = Window::default()
            .with_size(1280, 720)
            .with_label("Victoria 3 Save Analyzer");

        let tab_edge_buffer = 5; // buffer between edge of window and tab box
        let tab_box_height = 25;
        let output_height = 40;
        let tabs = Tabs::default()
            .with_pos(wind.x() + tab_edge_buffer, wind.y() + tab_edge_buffer)
            .with_size(
                wind.w() - 2 * tab_edge_buffer,
                wind.h() - 3 * tab_edge_buffer - output_height,
            );
        let scan_tab = ScanTab::new(&tabs, s, tab_box_height);
        let draw_tab = DrawTab::new(&tabs, s, tab_box_height);
        let preview_tab = PreviewTab::new(&tabs, s, tab_box_height);
        tabs.end();

        let mut output_info = Output::default()
            .with_pos(tabs.x(), tabs.y() + tabs.h() + tab_edge_buffer)
            .with_size(tabs.w(), output_height);
        output_info.set_value(" Idle");

        wind.end();

        Self {
            wind,
            scan_tab,
            draw_tab,
            preview_tab,
            output_info
        }
    }
    fn set_output(&mut self, inp: &str) {
        self.output_info.set_value(inp);
    }
    fn show(&mut self) {
        self.wind.show()
    }
    fn clear_saves(&mut self) {
        self.draw_tab.clear_saves();
    }
}

pub fn run() -> Result<(), VicError> {

    let mut info = Info::new();
    let mut mapdrawer = MapDrawer::default();

    let mut app = App::default().with_scheme(Scheme::Gtk);
    let (s, r) = app::channel::<usize>();

    let mut wind = VicWindow::new(s);

    wind.show();

    let mut returned_error: Option<VicError>;

    while app.wait() {
        match r.recv() {
            Some(100) => {
                wind.set_output(" Drawing...");
                (returned_error, info, mapdrawer) = wind.draw_tab.draw(info, mapdrawer, &mut app)?;
                if wind.draw_tab.preview() && returned_error.is_none() {
                    wind.preview_tab.insert_image(wind.draw_tab.get_draw_to());
                }
                if let Some(e) = returned_error {
                    wind.set_output(&format!(" Idle - Draw Error: {}", e));
                } else {
                    wind.set_output(" Idle");
                }
            }
            Some(101) => {
                wind.set_output(" Drawing...");
                (returned_error, info, mapdrawer) =
                    wind.draw_tab.quick_draw_countries(info, mapdrawer, &mut app)?;
                if wind.draw_tab.preview() && returned_error.is_none() {
                    wind.preview_tab.insert_image(wind.draw_tab.get_draw_to());
                }
                if let Some(e) = returned_error {
                    println!("{:?}", e);
                    wind.set_output(&format!(" Idle - Draw Error: {}", e));
                } else {
                    wind.set_output(" Idle");
                }
            }
            Some(102) => {
                wind.set_output(" Drawing...");
                (returned_error, info, mapdrawer) =
                    wind.draw_tab.quick_draw_states(info, mapdrawer, &mut app)?;
                if wind.draw_tab.preview() && returned_error.is_none() {
                    wind.preview_tab.insert_image(wind.draw_tab.get_draw_to());
                }
                if let Some(e) = returned_error {
                    println!("{:?}", e);
                    wind.set_output(&format!(" Idle - Draw Error: {}", e));
                } else {
                    wind.set_output(" Idle");
                }
            }
            Some(103) => wind.draw_tab.lock(),
            Some(104) => wind.draw_tab.check_custom_color(false), // from custom_color textbox callback
            Some(105) => wind.draw_tab.check_custom_color(true),
            Some(106) => wind.draw_tab.check_default_color(false), // from custom_default textbox callback
            Some(107) => wind.draw_tab.check_default_color(true),
            Some(108) => wind.draw_tab.check_custom_watercolor(false), // from custom_default textbox callback
            Some(109) => wind.draw_tab.check_custom_watercolor(true),
            Some(200) => {
                wind.set_output(" Scanning save...");
                wind.scan_tab.update(DataTypes::Save, PathBuf::new(), false);
                (info, _) = match info.find_path(DataTypes::Save, &mut app, &mut wind.scan_tab)? {
                    (Some(e), returned_info, new_save_path) => {
                        println!("{:?}", e);
                        wind.set_output(&format!(" Idle - Save Scan Error: {}", e));
                        (returned_info, new_save_path)
                    }
                    (None, returned_info, new_save_path) => {
                        wind.set_output(" Idle");
                        wind.draw_tab.add_save(new_save_path.as_path());
                        (returned_info, new_save_path)
                    }
                }
            }
            Some(201) => {
                wind.set_output(" Scanning game files...");
                wind.scan_tab.update(DataTypes::Map, PathBuf::new(), false);
                info.clear(DataTypes::Map);
                (info, _) = match info.find_path(DataTypes::Map, &mut app, &mut wind.scan_tab)? {
                    (Some(e), returned_info, new_map_path) => {
                        println!("{:?}", e);
                        wind.set_output(&format!(" Idle - Game Scan Error: {}", e));
                        (returned_info, new_map_path)
                    }
                    (None, returned_info, new_map_path) => {
                        wind.set_output(" Idle");
                        mapdrawer.set_path(new_map_path.clone());
                        (returned_info, new_map_path)
                    }
                }
            }
            Some(202) => {
                wind.clear_saves();
                wind.scan_tab.update(DataTypes::Save, PathBuf::new(), false);
                mapdrawer.clear();
                info.clear(DataTypes::Save);
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
        app: &mut App,
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
        thread::sleep(std::time::Duration::from_millis(10));

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
                    break 'outer Ok((
                        c.0,
                        c.1,
                        dialog.filename(),
                    ));
                }
            }
            Err(VicError::temp())
        }
    }
}
