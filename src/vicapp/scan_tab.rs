use fltk::{
    button::{Button, CheckButton},
    group::{Group, Pack, Tabs},
    output::Output,
    prelude::{GroupExt, InputExt, WidgetExt},
};

pub struct ScanTab {
    save_check: CheckButton,
    game_check: CheckButton,
    save_text: Output,
    game_text: Output,
}

impl ScanTab {
    pub fn new(tab: &Tabs, s: fltk::app::Sender<usize>) -> Self {
        let tab_box_height = 25; // height of box
        let button_width = 140; // width of button
        let button_height = 40; // height of button
        let checkbox_width = 20 + 0; // 20 is width of the actual box. The second number is space between checkbox and button.
        let edge_buffer = 10; // space between edge of box and button
        let spacing = 10; // space between each box
        let textbox_buffer = 10; // space between button and textbox

        let scan_group = Group::default()
            .with_pos(tab.x(), tab.y() + tab_box_height)
            .with_size(tab.w(), tab.h() - tab_box_height)
            .with_label("Tab1\t\t");

        // -----------------------------------

        let mut scan_buttons = Pack::default()
            .with_pos(
                scan_group.x() + checkbox_width + edge_buffer,
                scan_group.y() + edge_buffer,
            )
            .with_size(button_width, scan_group.h() - 2 * edge_buffer);

        scan_buttons.set_spacing(spacing);

        let mut game_selector = Button::default()
            .with_label("Load Game Files")
            .with_size(button_width, button_height);
        let mut save_selector = Button::default()
            .with_label("Load Save")
            .with_size(button_width, button_height);

        scan_buttons.end();

        // -----------------------------------

        let mut scan_checkbox = Pack::default()
            .with_pos(scan_group.x() + edge_buffer, scan_group.y() + edge_buffer)
            .with_size(checkbox_width, scan_group.h() - 2 * edge_buffer);

        scan_checkbox.set_spacing(spacing);

        let mut game_check = CheckButton::default().with_size(checkbox_width, button_height);
        let mut save_check = CheckButton::default().with_size(checkbox_width, button_height);

        scan_checkbox.end();

        // -----------------------------------

        let mut scan_textbox = Pack::default()
            .with_pos(
                scan_group.x() + checkbox_width + edge_buffer + textbox_buffer + button_width,
                scan_group.y() + edge_buffer,
            )
            .with_size(
                scan_group.w() - 2 * edge_buffer - checkbox_width - button_width - textbox_buffer,
                scan_group.h() - 2 * edge_buffer,
            );

        scan_textbox.set_spacing(spacing);

        let game_text = Output::default().with_size(
            scan_group.w() - 2 * edge_buffer - checkbox_width - button_width - textbox_buffer,
            button_height,
        );
        let save_text = Output::default().with_size(
            scan_group.w() - 2 * edge_buffer - checkbox_width - button_width - textbox_buffer,
            button_height,
        );

        scan_textbox.end();

        // -----------------------------------

        scan_group.end();

        save_selector.emit(s, 3);
        game_selector.emit(s, 4);

        save_check.set_callback(move |x| x.set_checked(!x.is_checked()));
        game_check.set_callback(move |x| x.set_checked(!x.is_checked()));

        Self {
            save_check,
            game_check,
            save_text,
            game_text,
        }
    }
    pub fn update_save(&mut self, label: &str, new_check: bool) {
        self.save_check.set_checked(new_check);
        self.save_text.set_value(label);
    }
    pub fn update_game(&mut self, label: &str, new_check: bool) {
        self.game_check.set_checked(new_check);
        self.game_text.set_value(label);
    }
}
