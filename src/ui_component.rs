use crate::util::Size;

//TODO: refactor/remove -> NB set_size not used in Message_bar
pub trait UIComponent {
    fn needs_redraw(&self) -> bool;

    fn set_needs_redraw(&mut self, value: bool);

    fn set_size(&mut self, size: Size);

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }
}
