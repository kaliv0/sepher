use std::time::Instant;

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
}
