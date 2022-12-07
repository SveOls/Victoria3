use std::{path::{PathBuf, Path}, thread};

use fltk::{
    button::{Button, CheckButton},
    group::{Group, Pack, Tabs},
    output::Output,
    prelude::{GroupExt, InputExt, WidgetExt, WidgetBase, MenuExt},
    menu::Choice,
    input::Input,
    app,
    app::{Sender, App},
    enums::Align
};

use crate::{draw::{MapDrawer, Coloring, self}, data::Info, wrappers::ColorWrap, error::VicError};


pub struct DrawTab {
    draw_data: Choice,
    choicecolor: Choice,
    choicelines: Choice,
    light_mode: Choice,
    choicenum: Choice,
    choiceden: Choice,
    input_name: Input,
}

impl DrawTab {
    pub fn new(tab: &Tabs, s: Sender<usize>, tab_box_height: i32) -> Self {

        let selection_height = 30;
        let selection_separation = 15;
        let edge_buffer = 10;
        let label_height = 15; // note: if changed from align::top, things might look weird. take care.
        let selection_width = 140;
        let label_align_choice = Align::Top;
        // let label_align_check = Align::Center;



        let draw_group = Group::default()
            .with_pos(tab.x(), tab.y() + tab_box_height)
            .with_size(tab.w(), tab.h() - tab_box_height)
            .with_label("Tab2\t\t");

        let mut draw_buttons = Pack::default()
            .with_pos(
                draw_group.x() + edge_buffer,
                draw_group.y() + edge_buffer + label_height,
            )
            .with_size(selection_width, draw_group.h() - 2 * edge_buffer);

        draw_buttons.set_spacing(label_height + selection_separation);

        let mut choiceden = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("Select denom")
            .with_align(label_align_choice);
        let mut choicenum = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("Select numer")
            .with_align(label_align_choice);
        let input_name = Input::default()
            .with_size(selection_width, selection_height)
            .with_label("name")
            .with_align(label_align_choice);
        let mut choicecolor = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("Select color")
            .with_align(label_align_choice);
        let mut choicelines = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("Select lines")
            .with_align(label_align_choice);


        let mut light_mode = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("light mode")
            .with_align(label_align_choice);
        let mut draw_data = Choice::default()
            .with_size(selection_width, selection_height)
            .with_label("data mode")
            .with_align(label_align_choice);

        // let light_mode = CheckButton::default()
        //     .with_size(selection_width, (selection_height - label_height).min(20))
        //     .with_label("light mode")
        //     .with_align(label_align_check);
        // let draw_data = CheckButton::default()
        //     .with_size(selection_width, (selection_height - label_height).min(20))
        //     .with_label("data mode")
        //     .with_align(label_align_check);

        draw_buttons.end();

        // btu.set_trigger(CallbackTrigger::EnterKeyAlways);
        let mut engage = Button::new(560, 420, 160, 40, "Draw");
        let mut quickdraw_states = Button::new(560, 20, 160, 40, "(fast) Draw States");
        let mut quickdraw_countries = Button::new(560, 80, 160, 40, "(fast) Draw Countries");

        // let light_mode = CheckButton::new(380, 360, 100, 40, "light mode");
        // let draw_data = CheckButton::new(380, 420, 100, 40, "data");

        // let mut choiceden =
        //     Choice::new(550, 145, 150, 30, "Select denom").with_align(Align::PositionMask);
        // let mut choicenum =
        //     Choice::new(550, 200, 150, 30, "Select numer").with_align(Align::Wrap);
        // let mut choicecolor =
        //     Choice::new(550, 255, 150, 30, "Select color").with_align(Align::LeftBottom);
        // let mut choicelines =
        //     Choice::new(550, 310, 150, 30, "Select lines").with_align(Align::LeftTop);



        draw_group.end();

        choicenum.add_choice(" religion| culture| population");
        choiceden.add_choice(" None| population| area");
        choicecolor.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
        choicelines.add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
        draw_data.add_choice(" yes| no");
        light_mode.add_choice(" yes| no");

        choiceden.set_value(0);
        choicecolor.set_value(3);
        choicelines.set_value(3);
        choicenum.set_value(0);

        engage.emit(s, 0);
        quickdraw_countries.emit(s, 1);
        quickdraw_states.emit(s, 2);

        Self {
            draw_data,
            choicecolor,
            light_mode,
            choicenum,
            choiceden,
            choicelines,
            input_name
        }
    }
    pub fn temp_fix_savestates(&mut self) {
        if self.draw_data.value() == 0 {
            self.choicecolor.set_value(3);
        }
    }
    pub fn draw(&mut self, info: Info, mut mapdrawer: MapDrawer, app: &mut App) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {

        println!("one");
        if let Err(e) = self.draw_inner(&info, &mut mapdrawer) {
            return Ok((Some(e), info, mapdrawer))
        }
        println!("two");

        let (s, r) = app::channel::<(Option<VicError>, Info, MapDrawer)>();

        thread::spawn({
            let draw_data = self.draw_data.value() == 0;
            move || {
                s.send((mapdrawer.draw(&info, PathBuf::from("output"), draw_data).err(), info, mapdrawer))
            }
        });

        while app.wait() {
            if let Some(a) = r.recv() {
                println!("draw complete");
                return Ok(a)
            }
        }
        Err(VicError::named("ran out of app while drawing i guess"))
    }
    fn draw_inner(&mut self, info: &Info, mapdrawer: &mut MapDrawer) -> Result<(), VicError> {
        //
        self.temp_fix_savestates();
        //
        let (num, col) = match self.choicenum.value() {
            0 => info.religion(&self.input_name.value())?,
            1 => info.culture(&self.input_name.value())?,
            2 if self.light_mode.value() == 0 => (
                info.population()?,
                Some(ColorWrap::from(image::Rgb::from([0x00, 0x00, 0x00]))),
            ),
            2 if self.light_mode.value() == 1 => (
                info.population()?,
                Some(ColorWrap::from(image::Rgb::from([0xFF, 0xFF, 0xFF]))),
            ),
            _ => return Err(VicError::temp()),
        };
        mapdrawer.set_numerator(Some(num));
        mapdrawer.set_color(col);
        match self.choicecolor.value() {
            0 => mapdrawer.set_color_map(Coloring::None),
            1 => mapdrawer.set_color_map(Coloring::Provinces),
            2 => mapdrawer.set_color_map(Coloring::StateTemplates),
            3 => mapdrawer.set_color_map(Coloring::SaveStates),
            4 => mapdrawer.set_color_map(Coloring::SaveCountries),
            _ => {}
        }
        match self.choicelines.value() {
            0 => mapdrawer.set_lines(Coloring::None),
            1 => mapdrawer.set_lines(Coloring::Provinces),
            2 => mapdrawer.set_lines(Coloring::StateTemplates),
            3 => mapdrawer.set_lines(Coloring::SaveStates),
            4 => mapdrawer.set_lines(Coloring::SaveCountries),
            _ => {}
        }
        match self.choiceden.value() {
            0 => mapdrawer.set_denominator(None),
            1 => mapdrawer.set_denominator(Some(info.population()?)),
            2 => mapdrawer.set_denominator(Some(info.area()?)),
            _ => {}
        }
        mapdrawer.darkmode(self.light_mode.value() == 1);
        mapdrawer.set_sea_color(ColorWrap::from(image::Rgb::from([0, 100, 200])));

        Ok(())
    }
    pub fn quick_draw_countries(&mut self, info: Info, mapdrawer: MapDrawer, app: &mut App) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {
        self.choicecolor.set_value(4);
        self.draw_data.set_value(1);
        self.draw(info, mapdrawer, app)
    }
    pub fn quick_draw_states(&mut self, info: Info, mapdrawer: MapDrawer, app: &mut App) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {
        self.choicecolor.set_value(3);
        self.draw_data.set_value(1);
        self.draw(info, mapdrawer, app)
    }
}
