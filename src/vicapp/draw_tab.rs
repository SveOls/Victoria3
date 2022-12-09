use super::*;

pub struct DrawTab {
    draw_data: Choice,
    choicecolor: Choice,
    choicelines: Choice,
    default_color: Choice,
    choicenum: Choice,
    choiceden: Choice,
    input_name: Input,
    custom_color: Input,
    custom_default: Input,
    custom_color_check: CheckButton,
    custom_default_check: CheckButton,
    valscale_num: FloatInput,
    choice_valscale: Choice,
    choice_watercolor: Choice,
    custom_watercolor: Input,
    custom_watercolor_check: CheckButton,
    choice_waterlines: Choice,
    status: Output,
    preview_selector: Choice,
    savepath: Input,
    save_selector: Choice,
}

impl DrawTab {
    pub fn new(tab: &Tabs, s: Sender<usize>, tab_box_height: i32) -> Self {
        let selection_height = 30;
        let selection_separation = 15;
        let edge_buffer = 10;
        let label_height = 15; // note: if changed from align::top, things might look weird. take care.
        let selection_width = 200;
        let checkbox_width = 20;
        let button_width = 200;
        let button_separation = 10;
        let button_height = 60;
        let label_align_choice = Align::Top;
        // let label_align_check = Align::Center;

        let draw_group = Group::default()
            .with_pos(tab.x(), tab.y() + tab_box_height)
            .with_size(tab.w(), tab.h() - tab_box_height)
            .with_label("Drawing \t");

        let mut t_y_left = draw_group.y() + edge_buffer + label_height;
        let mut t_y_right = draw_group.y() + edge_buffer;

        let mut choicecolor = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("Select color")
            .with_align(label_align_choice);
        t_y_left += selection_separation + choicecolor.h() + label_height;

        let mut choice_watercolor = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("Select water color")
            .with_align(label_align_choice);
        let mut custom_watercolor = Input::default()
            .with_pos(
                draw_group.x() + edge_buffer + selection_width + selection_separation,
                t_y_left,
            )
            .with_size(selection_width - checkbox_width, selection_height)
            .with_label("custom sea color")
            .with_align(label_align_choice);
        let mut custom_watercolor_check = CheckButton::default()
            .with_pos(
                draw_group.x() + edge_buffer + 2 * selection_width + selection_separation
                    - checkbox_width,
                t_y_left,
            )
            .with_size(checkbox_width, selection_height);
        t_y_left += selection_separation + choice_watercolor.h() + label_height;

        let mut choicelines = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("Select lines")
            .with_align(label_align_choice);
        t_y_left += selection_separation + choicelines.h() + label_height;

        let mut choice_waterlines = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("Sea Province Line")
            .with_align(label_align_choice);
        t_y_left += selection_separation + choice_waterlines.h() + label_height;

        let mut draw_data = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("data mode")
            .with_align(label_align_choice);
        let mut custom_color = Input::default()
            .with_pos(
                draw_group.x() + edge_buffer + selection_width + selection_separation,
                t_y_left,
            )
            .with_size(selection_width - checkbox_width, selection_height)
            .with_label("custom color")
            .with_align(label_align_choice);
        let mut custom_color_check = CheckButton::default()
            .with_pos(
                draw_group.x() + edge_buffer + 2 * selection_width + selection_separation
                    - checkbox_width,
                t_y_left,
            )
            .with_size(checkbox_width, selection_height);
        t_y_left += selection_separation + draw_data.h() + label_height;

        let mut default_color = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("default color")
            .with_align(label_align_choice);
        let mut custom_default = Input::default()
            .with_pos(
                draw_group.x() + edge_buffer + selection_width + selection_separation,
                t_y_left,
            )
            .with_size(selection_width - checkbox_width, selection_height)
            .with_label("custom default")
            .with_align(label_align_choice);
        let mut custom_default_check = CheckButton::default()
            .with_pos(
                draw_group.x() + edge_buffer + 2 * selection_width + selection_separation
                    - checkbox_width,
                t_y_left,
            )
            .with_size(checkbox_width, selection_height);
        t_y_left += selection_separation + default_color.h() + label_height;

        let mut choicenum = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("Select data source")
            .with_align(label_align_choice);

        let mut input_name = Input::default()
            .with_pos(
                draw_group.x() + edge_buffer + selection_width + selection_separation,
                t_y_left,
            )
            .with_size(selection_width, selection_height)
            .with_label("name (if applicable)")
            .with_align(label_align_choice);
        t_y_left += selection_separation + choicenum.h() + label_height;

        let mut choiceden = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("scale to")
            .with_align(label_align_choice);
        t_y_left += selection_separation + choiceden.h() + label_height;

        let mut choice_valscale = Choice::default()
            .with_pos(draw_group.x() + edge_buffer, t_y_left)
            .with_size(selection_width, selection_height)
            .with_label("number scaling")
            .with_align(label_align_choice);

        let mut valscale_num = FloatInput::default()
            .with_pos(
                draw_group.x() + edge_buffer + selection_width + selection_separation,
                t_y_left,
            )
            .with_size(selection_width, selection_height)
            .with_label("num (if applicable)")
            .with_align(label_align_choice);

        // draw_buttons.end();

        let mut engage = Button::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, button_height)
            .with_label("Draw");
        t_y_right += button_separation + engage.h();

        let mut quickdraw_states = Button::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, button_height)
            .with_label("(fast) Draw States");
        t_y_right += button_separation + quickdraw_states.h();

        let mut quickdraw_countries = Button::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, button_height)
            .with_label("(fast) Draw Countries");
        t_y_right += button_separation + quickdraw_countries.h();

        t_y_right += label_height;
        let save_selector = Choice::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, selection_height)
            .with_align(label_align_choice)
            .with_label("save selector");
        t_y_right += button_separation + save_selector.h();

        t_y_right += label_height;
        let mut preview_selector = Choice::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, selection_height)
            .with_align(label_align_choice)
            .with_label("Preview");
        t_y_right += button_separation + preview_selector.h();

        t_y_right += label_height;
        let mut savepath = Input::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, selection_height)
            .with_label("save to")
            .with_align(label_align_choice);
        t_y_right += button_separation + valscale_num.h();

        t_y_right += label_height;
        let mut status = Output::default()
            .with_pos(
                draw_group.x() + draw_group.w() - edge_buffer - button_width,
                t_y_right,
            )
            .with_size(button_width, selection_height)
            .with_align(label_align_choice)
            .with_label("Draw Status");

        // let mut quickdraw_states = Button::new(560, 20, 160, 40, "(fast) Draw States");
        // let mut quickdraw_countries = Button::new(560, 80, 160, 40, "(fast) Draw Countries");

        draw_group.end();

        // -------------------
        // Settings
        // -------------------
        {
            // ---------------
            // Coloring Settings
            // ---------------
            {
                choicecolor
                    .add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
                choicecolor.set_value(3);
                // -----
                choice_watercolor.add_choice(" Default| Custom| As Land");
                choice_watercolor.set_value(0);
                choice_watercolor.emit(s, 103);
                //
                custom_watercolor.set_trigger(CallbackTrigger::Changed);
                custom_watercolor.set_callback(move |x| Self::col_box_callback(x, s, 108));
                //
                custom_watercolor_check.set_callback(read_only_checkbutton);
                // -----
                choicelines
                    .add_choice(" None| Provinces| StateTemplate| SaveStates| SaveCountries");
                choicelines.set_value(3);
                //
                choice_waterlines.add_choice(" Yes| No");
                choice_waterlines.set_value(1);
            }
            // ---------------
            // Data-Color Settings
            // ---------------
            {
                draw_data.add_choice(
                    " yes (default color)| yes (amplified default)| yes (custom color)| no",
                );
                draw_data.set_value(3);
                draw_data.emit(s, 103);
                //
                custom_color.set_trigger(CallbackTrigger::Changed);
                custom_color.set_callback(move |x| Self::col_box_callback(x, s, 104));
                //
                custom_color_check.set_callback(read_only_checkbutton);
                // -----
                default_color.add_choice(" white| black| custom");
                default_color.set_value(0);
                default_color.emit(s, 103);
                //
                custom_default.set_trigger(CallbackTrigger::Changed);
                custom_default.set_callback(move |x| Self::col_box_callback(x, s, 106));
                //
                custom_default_check.set_callback(read_only_checkbutton);
            }
            // ---------------
            // Data Settings
            // ---------------
            {
                choicenum.add_choice(" religion| culture| population");
                choicenum.set_value(0);
                choicenum.emit(s, 103); // emit 103 => call DrawTab.lock()
                                        //
                input_name.set_trigger(CallbackTrigger::Changed);
                input_name.set_callback(move |x| x.set_value(&x.value())); // allows using .do_callback() for updating textcolor
                                                                           // -----
                choiceden.add_choice(" total| population| area");
                choiceden.set_value(0);
                // -----
                choice_valscale.add_choice(" none| polynomial_n");
                choice_valscale.set_value(0);
                choice_valscale.emit(s, 103);
                //
                valscale_num.set_trigger(CallbackTrigger::Changed);
                valscale_num.set_callback(move |x| x.set_value(&x.value())); // allows using .do_callback() for updating textcolor
            }
            // ---------------
            // Draw Settings
            // ---------------
            {
                engage.emit(s, 100);
                // -----
                quickdraw_countries.emit(s, 101);
                // -----
                quickdraw_states.emit(s, 102);
                // -----
                preview_selector.add_choice(" Yes| No");
                preview_selector.set_value(0);
                // -----
                savepath.set_value("test");
                // -----
                status.set_value("idle");
            }
            // ---------------
        }
        // -------------------

        let mut ret = Self {
            draw_data,
            choicecolor,
            default_color,
            choicenum,
            choiceden,
            choicelines,
            input_name,
            custom_color,
            custom_default,
            custom_color_check,
            custom_default_check,
            valscale_num,
            choice_valscale,
            choice_watercolor,
            choice_waterlines,
            custom_watercolor,
            custom_watercolor_check,
            status,
            preview_selector,
            savepath,
            save_selector,
        };
        ret.lock();
        ret
    }
    /// callback function for Input, limiting to 6-character hex and sending the message
    /// s(sendloc) when the box has 0-5 chars, and s(sendloc+1) when it has 6 (the max).
    fn col_box_callback(x: &mut Input, s: Sender<usize>, sendloc: usize) {
        x.set_value(
            &x.value()
                .char_indices()
                .filter(|(_, c)| c.is_ascii_hexdigit())
                .map(|(i, c)| (i, c.to_ascii_uppercase()))
                .inspect(|&(i, _)| s.send(sendloc + (i == 5) as usize))
                .map(|(_, c)| c)
                .take(6)
                .collect::<String>(),
        );
    }
    pub fn add_save(&mut self, path: &Path) {
        self.save_selector
            .add_choice(path.file_name().unwrap().to_string_lossy().as_ref());
    }
    pub fn clear_saves(&mut self) {
        self.save_selector.clear()
    }
    pub fn preview(&self) -> bool {
        self.preview_selector.value() == 0
    }
    pub fn lock(&mut self) {
        if let 0 | 1 = self.choicenum.value() {
            self.input_name.set_readonly(false);
            self.input_name.set_text_color(Color::from_hex(0x000000));
        } else {
            self.input_name.set_readonly(true);
            self.input_name.set_text_color(Color::from_hex(0x999999));
        }
        if let 1 = self.choice_watercolor.value() {
            self.custom_watercolor.set_readonly(false);
            self.custom_watercolor
                .set_text_color(Color::from_hex(0x000000));
        } else {
            self.custom_watercolor.set_readonly(true);
            self.custom_watercolor
                .set_text_color(Color::from_hex(0x999999));
        }
        if let 2 = self.draw_data.value() {
            self.custom_color.set_readonly(false);
            self.custom_color.set_text_color(Color::from_hex(0x000000));
        } else {
            self.custom_color.set_readonly(true);
            self.custom_color.set_text_color(Color::from_hex(0x999999));
        }
        if let 2 = self.default_color.value() {
            self.custom_default.set_readonly(false);
            self.custom_default
                .set_text_color(Color::from_hex(0x000000));
        } else {
            self.custom_default.set_readonly(true);
            self.custom_default
                .set_text_color(Color::from_hex(0x999999));
        }
        if let 1 = self.choice_valscale.value() {
            self.valscale_num.set_readonly(false);
            self.valscale_num.set_text_color(Color::from_hex(0x000000));
        } else {
            self.valscale_num.set_readonly(true);
            self.valscale_num.set_text_color(Color::from_hex(0x999999));
        }
        self.valscale_num.do_callback();
        self.input_name.do_callback();
        self.custom_default.do_callback();
        self.custom_color.do_callback();
        self.custom_watercolor.do_callback();
    }
    pub fn check_custom_color(&mut self, to: bool) {
        self.custom_color_check.set_value(to)
    }
    pub fn check_default_color(&mut self, to: bool) {
        self.custom_default_check.set_value(to)
    }
    pub fn check_custom_watercolor(&mut self, to: bool) {
        self.custom_watercolor_check.set_value(to)
    }
    fn savepath(&self) -> String {
        if self.savepath.value().trim().is_empty() {
            "none".to_owned()
        } else {
            self.savepath.value()
        }
    }
    pub fn get_draw_to(&self) -> PathBuf {
        let mut ret = PathBuf::from("output").join(self.savepath());
        ret.set_extension("png");
        ret
    }
    fn temp_fix_savestates(&mut self) {
        if self.is_data().unwrap() {
            self.choicecolor.set_value(3);
        }
    }
    fn is_data(&self) -> Result<bool, VicError> {
        match self.draw_data.value() {
            0 => Ok(true),
            1 => Ok(true),
            2 => Ok(true),
            3 => Ok(false),
            _ => Err(VicError::temp()),
        }
    }
    /// Ok(Some(error)) => error happened while preparing data or drawing map. Potentially recoverable, so Info and MapDrawer must be returned.
    /// Err(error) => app stopped before mapdraw finished drawing. Non-recoverable, so dropping Info and Mapdrawer is okay
    /// might make a new VicError type for this kind of situation.
    pub fn draw(
        &mut self,
        info: Info,
        mut mapdrawer: MapDrawer,
        app: &mut App,
    ) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {
        self.status.set_value("preparing draw settings");
        if self.save_selector.value() < 0 {
            self.status.set_value("no save selected");
            return Ok((Some(VicError::temp()), info, mapdrawer));
        }
        if let Err(e) = self.draw_inner(&info, &mut mapdrawer) {
            self.status.set_value("failed at preparing data");
            return Ok((Some(e), info, mapdrawer));
        }
        self.status.set_value("drawing map...");

        let (s, r) = app::channel::<(Option<VicError>, Info, MapDrawer)>();

        thread::spawn({
            let is_data = match self.is_data() {
                Ok(a) => a,
                Err(e) => return Ok((Some(e), info, mapdrawer)),
            };
            let mut savepath = PathBuf::from("output").join(self.savepath());
            let save_selector = self.save_selector.value() as usize;
            savepath.set_extension("png");
            move || {
                s.send((
                    mapdrawer
                        .draw(&info, save_selector, savepath, is_data)
                        .err(),
                    info,
                    mapdrawer,
                ))
            }
        });

        while app.wait() {
            if let Some(a) = r.recv() {
                self.status.set_value("idle");
                return Ok(a);
            }
        }
        Err(VicError::named("ran out of app while drawing i guess"))
    }
    fn draw_inner(&mut self, info: &Info, mapdrawer: &mut MapDrawer) -> Result<(), VicError> {
        //
        self.temp_fix_savestates();
        //
        match self.choicecolor.value() {
            0 => mapdrawer.set_color_map(Coloring::None),
            1 => mapdrawer.set_color_map(Coloring::Provinces),
            2 => mapdrawer.set_color_map(Coloring::StateTemplates),
            3 => mapdrawer.set_color_map(Coloring::SaveStates),
            4 => mapdrawer.set_color_map(Coloring::SaveCountries),
            _ => return Err(VicError::temp()),
        }
        match self.choicelines.value() {
            0 => mapdrawer.set_lines(Coloring::None),
            1 => mapdrawer.set_lines(Coloring::Provinces),
            2 => mapdrawer.set_lines(Coloring::StateTemplates),
            3 => mapdrawer.set_lines(Coloring::SaveStates),
            4 => mapdrawer.set_lines(Coloring::SaveCountries),
            _ => return Err(VicError::temp()),
        }
        match self.choice_watercolor.value() {
            0 => mapdrawer.set_sea_color(Some(ColorWrap::to_colorwrap("x002266")?)),
            1 if self.custom_watercolor_check.is_checked() => mapdrawer.set_sea_color(Some(
                ColorWrap::to_colorwrap(&self.custom_watercolor.value())?,
            )),
            2 => mapdrawer.set_sea_color(None),
            _ => return Err(VicError::temp()),
        }
        match self.choice_waterlines.value() {
            0 => mapdrawer.sea_province_borders(true),
            1 => mapdrawer.sea_province_borders(false),
            _ => return Err(VicError::temp()),
        }
        if !self.is_data()? {
            return Ok(());
        }
        match self.default_color.value() {
            0 => mapdrawer.darkmode(ColorWrap::to_colorwrap("xFFFFFF")?),
            1 => mapdrawer.darkmode(ColorWrap::to_colorwrap("x000000")?),
            2 if self.custom_default_check.is_checked() => {
                mapdrawer.darkmode(ColorWrap::to_colorwrap(&self.custom_default.value())?)
            }
            _ => return Err(VicError::temp()),
        }
        let (num, col) = match self.choicenum.value() {
            0 => info.religion(
                &self.input_name.value(),
                self.save_selector.value() as usize,
            )?,
            1 => info.culture(
                &self.input_name.value(),
                self.save_selector.value() as usize,
            )?,
            2 => (info.population(self.save_selector.value() as usize)?, None),
            _ => return Err(VicError::temp()),
        };
        mapdrawer.set_numerator(Some(num));
        match self.choiceden.value() {
            0 => mapdrawer.set_denominator(None),
            1 => mapdrawer
                .set_denominator(Some(info.population(self.save_selector.value() as usize)?)),
            2 => mapdrawer.set_denominator(Some(info.area(self.save_selector.value() as usize)?)),
            _ => return Err(VicError::temp()),
        }
        match self.draw_data.value() {
            0 => mapdrawer.set_color(col),
            1 => mapdrawer.set_color(col),
            2 if self.custom_color_check.is_checked() => {
                mapdrawer.set_color(Some(ColorWrap::to_colorwrap(&self.custom_color.value())?))
            }
            3 => return Ok(()),
            _ => return Err(VicError::temp()),
        }
        match self.choice_valscale.value() {
            0 => mapdrawer.set_data_scale(None),
            1 => mapdrawer.set_data_scale({
                match self.valscale_num.value().parse() {
                    Ok(a) => Some((|x, y| x.powf(y), a)),
                    Err(_) => None,
                }
            }),
            _ => return Err(VicError::temp()),
        }
        mapdrawer.extremify(self.draw_data.value() == 1);

        Ok(())
    }
    pub fn quick_draw_countries(
        &mut self,
        info: Info,
        mapdrawer: MapDrawer,
        app: &mut App,
    ) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {
        self.choicecolor.set_value(4);
        self.draw_data.set_value(3);
        self.draw(info, mapdrawer, app)
    }
    pub fn quick_draw_states(
        &mut self,
        info: Info,
        mapdrawer: MapDrawer,
        app: &mut App,
    ) -> Result<(Option<VicError>, Info, MapDrawer), VicError> {
        self.choicecolor.set_value(3);
        self.draw_data.set_value(3);
        self.draw(info, mapdrawer, app)
    }
}
