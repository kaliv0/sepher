use crate::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height - 1 {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(current_row) {
                // Terminal::print(line)?;
                // Terminal::print("\r\n")?;

                //TODO: when refactoring consider changing printing welcome msg and tildes with \r\n as separate function
                Terminal::print(format!("{line}\r\n").as_str())?;
                continue;
            }

            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or down
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                //TODO: should we borrow or move string into print?
                Terminal::print(&Self::get_welcome_message()?)?;
            } else {
                Terminal::print("~\r\n")?;
            }
        }
        Terminal::print("~")?;
        Ok(())
    }

    fn get_welcome_message() -> Result<String, Error> {
        let mut welcome_message =
            format!("No, it's not VIM -- {} v{}", NAME.to_uppercase(), VERSION);
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}\r\n");
        welcome_message.truncate(width);
        Ok(welcome_message)
    }
}

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}
