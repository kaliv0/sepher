use crate::terminal::Terminal;
use crate::util::Size;
use std::io::Error;
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

    pub fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    pub fn render(&mut self, origin_row: usize) {
        if self.needs_redraw() {
            if let Err(err) = self.draw(origin_row) {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not render component: {err:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = err;
                }
            } else {
                self.set_needs_redraw(false);
            }
        }
    }

    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        if self.current_message.is_expired() {
            // Upon expiration, we need to write out "" once to clear the message.
            // To avoid clearing more than necessary, we  keep track of the fact that we've already cleared the expired message once.
            self.cleared_after_expiry = true;
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin, message)
    }
}
