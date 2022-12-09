use super::*;

pub struct ScanTab {
    save_check: CheckButton,
    game_check: CheckButton,
    save_text: Output,
    save_text_pathbuf: PathBuf,
    game_text: Output,
    game_text_pathbuf: PathBuf,
}

impl ScanTab {
    pub fn new(tab: &Tabs, s: app::Sender<usize>, tab_box_height: i32) -> Self {
        //let tab_box_height = height of box
        let button_width = 140; // width of button
        let button_height = 40; // height of button
        let checkbox_width = 20 + 0; // 20 is width of the actual box. The second number is space between checkbox and button.
        let edge_buffer = 10; // space between edge of box and button
        let spacing = 10; // space between each box
        let textbox_buffer = 10; // space between button and textbox

        let scan_group = Group::default()
            .with_pos(tab.x(), tab.y() + tab_box_height)
            .with_size(tab.w(), tab.h() - tab_box_height)
            .with_label("Scanning\t");

        // -----------------------------------

        let mut ty = scan_group.y() + edge_buffer;
        let mut game_check = CheckButton::default()
            .with_pos(scan_group.x() + edge_buffer, ty)
            .with_size(checkbox_width, button_height);
        let mut game_selector = Button::default()
            .with_pos(game_check.x() + game_check.w(), ty)
            .with_size(button_width, game_check.h())
            .with_label("Load Game Files");
        let game_text = Output::default()
            .with_pos(game_selector.x() + game_selector.w() + textbox_buffer, ty)
            .with_size(
                scan_group.w() - game_selector.x() - game_selector.w() - edge_buffer - textbox_buffer,
                game_selector.h(),
            );
        ty += spacing + game_text.h();

        let mut save_check = CheckButton::default()
            .with_pos(scan_group.x() + edge_buffer, ty)
            .with_size(checkbox_width, button_height);
        let mut save_selector = Button::default()
            .with_pos(save_check.x() + save_check.w(), ty)
            .with_size(button_width, save_check.h())
            .with_label("Load Save");
        let save_text = Output::default()
            .with_pos(save_selector.x() + save_selector.w() + textbox_buffer, ty)
            .with_size(
                scan_group.w() - save_selector.x() - save_selector.w() - edge_buffer - textbox_buffer,
                save_selector.h(),
            );
        ty += spacing + save_text.h();

        let mut save_clear = Button::default()
            .with_pos(scan_group.x() + edge_buffer, ty)
            .with_size(button_width + checkbox_width, button_height)
            .with_label("Clear Saves");

        // -----------------------------------

        scan_group.end();

        save_selector.emit(s, 200);
        game_selector.emit(s, 201);
        save_clear.emit(s, 202);

        save_check.set_callback(read_only_checkbutton);
        game_check.set_callback(read_only_checkbutton);
        Self {
            save_check,
            game_check,
            save_text,
            save_text_pathbuf: PathBuf::new(),
            game_text,
            game_text_pathbuf: PathBuf::new(),
        }
    }
    pub fn update(&mut self, datatype: DataTypes, label: PathBuf, new_check: bool) {
        match datatype {
            DataTypes::Map => {
                self.game_check.set_checked(new_check);
                self.game_text_pathbuf = label;
                self.game_text
                    .set_value(self.game_text_pathbuf.to_string_lossy().as_ref());
            }
            DataTypes::Save => {
                self.save_check.set_checked(new_check);
                self.save_text_pathbuf = label;
                self.save_text
                    .set_value(self.save_text_pathbuf.to_string_lossy().as_ref());
                self.save_text_pathbuf.pop();
            }
        }
    }
    pub fn path(&mut self, datatype: DataTypes) -> Option<&Path> {
        match datatype {
            DataTypes::Map => {
                if self.game_text_pathbuf.as_path() != Path::new("") {
                    Some(self.game_text_pathbuf.as_path())
                } else {
                    None
                }
            }
            DataTypes::Save => {
                if self.save_text_pathbuf.as_path() != Path::new("") {
                    Some(self.save_text_pathbuf.as_path())
                } else {
                    None
                }
            }
        }
    }
}
