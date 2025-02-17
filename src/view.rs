use crate::ui_component::UIComponent;
use crate::util::Size;

#[derive(Default)]
pub struct View {
    // buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    // text_location: Location,
    // scroll_offset: Position,
    // search_info: Option<SearchInfo>,
}

impl View {
    fn scroll_text_location_into_view(&mut self) {
        // let Position { row, col } = self.text_location_to_position();
        // self.scroll_vertically(row);
        // self.scroll_horizontally(col);
    }
}

impl UIComponent for View {
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
}
