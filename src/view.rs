use crate::buffer::Buffer;
use crate::document_status::DocumentStatus;
use crate::util::{Location, Size};
use std::io::Error;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    // scroll_offset: Position,
    // search_info: Option<SearchInfo>,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        let file_info = self.buffer.get_file_info(); //TODO: read directly file_info without calling getter
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_idx: self.text_location.line_idx,
            file_name: format!("{file_info}"), // TODO: use custom Display/fmt but still...
            is_modified: self.buffer.is_dirty(), // TODO: or just self.buffer.dirty ?
        }
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.set_needs_redraw(true);
        Ok(())
    }

    fn scroll_text_location_into_view(&mut self) {
        // let Position { row, col } = self.text_location_to_position();
        // self.scroll_vertically(row);
        // self.scroll_horizontally(col);
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    pub fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }
}
