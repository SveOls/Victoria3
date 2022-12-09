use super::*;

pub struct PreviewTab {
    image_frame: Frame,
    show_path: Output,
}

impl PreviewTab {
    pub fn new(tabs: &Tabs, _s: Sender<usize>, tab_box_height: i32) -> Self {
        let edge_buffer = 10;
        let textbox_height = 40;

        let preview_group = Group::default()
            .with_pos(tabs.x(), tabs.y() + tab_box_height)
            .with_size(tabs.w(), tabs.h() - tab_box_height)
            .with_label("Preview \t");

        let show_path = Output::default()
            .with_size(preview_group.w() - 2 * edge_buffer, textbox_height)
            .with_pos(
                preview_group.x() + edge_buffer,
                preview_group.y() + edge_buffer,
            );

        let image_frame = Frame::default()
            .with_size(
                preview_group.w() - 2 * edge_buffer,
                preview_group.h() - 3 * edge_buffer - show_path.h(),
            )
            .with_pos(
                preview_group.x() + edge_buffer,
                show_path.y() + show_path.h() + edge_buffer,
            );

        preview_group.end();

        Self {
            image_frame,
            show_path,
        }
    }
    pub fn insert_image(&mut self, at: PathBuf) {
        println!("{:?}", at);
        let mut image = PngImage::load(at.as_path()).unwrap();
        image.scale(self.image_frame.w(), self.image_frame.h(), true, false);
        self.image_frame.set_image(Some(image));
        self.show_path.set_value(at.to_string_lossy().as_ref());
    }
}
