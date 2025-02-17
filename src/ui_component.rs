use crate::util::Size;

pub trait UIComponent {
    fn needs_redraw(&self) -> bool;

    fn set_needs_redraw(&mut self, value: bool);

    fn set_size(&mut self, size: Size);

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }
}
