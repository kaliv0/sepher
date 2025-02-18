use crate::util::Size;
use std::time::{Duration, Instant};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
    text: String,
    time: Instant,
}

//TODO: test without explicit default
impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

///////////////////////////////////
#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    //TODO: do we need expiration timeout?
    cleared_after_expiry: bool, //ensures we can properly hide expired messages
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    //TODO: not LGTM -> basically dismissing this methods?!
    fn set_size(&mut self, _: Size) {}

    pub(crate) fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }
}
