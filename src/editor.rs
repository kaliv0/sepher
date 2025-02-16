use crate::terminal::{Position, Size, Terminal};
use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyModifiers};
use std::cmp::min;
use std::io::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location, // location of cursor within text
}

impl Editor {
    pub fn run(&mut self) {
        // TODO: move error handling inside main?
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            // kind: KeyEventKind::Press,  //NB: for Windows only
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        // TODO: refactor without destructuring location and creating new one at the end
        // let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                self.location.y = self.location.y.saturating_sub(1);
            }
            KeyCode::Down => {
                self.location.y = min(height.saturating_sub(1), self.location.y.saturating_add(1));
            }
            KeyCode::Left => {
                self.location.x = self.location.x.saturating_sub(1);
            }
            KeyCode::Right => {
                self.location.x = min(width.saturating_sub(1), self.location.x.saturating_add(1));
            }
            KeyCode::PageUp => {
                self.location.y = 0;
            }
            KeyCode::PageDown => {
                self.location.y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                self.location.x = 0;
            }
            KeyCode::End => {
                self.location.x = width.saturating_sub(1);
            }
            _ => (),
        }
        // self.location = Location { x, y };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            // TODO: move logic inside terminate??
            // https://stackoverflow.com/questions/78174550/crossterm-not-clearing-screen-properly
            Terminal::clear_screen()?;
            // Terminal::purge_screen()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height - 1 {
            Terminal::clear_line()?;
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or down
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                // Self::draw_empty_row()?;
                // Terminal::print("~\r\n")?;
                Terminal::print("~\r\n")?;
            }
            // if current_row.saturating_add(1) < height {
            //     Terminal::print("\r\n")?;
            // }
        }
        Terminal::print("~")?;
        Ok(())
    }

    //TODO: remove function
    // fn draw_empty_row() -> Result<(), Error> {
    //     Terminal::print("~")?;
    //     Ok(())
    // }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("No, it's not VIM -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}\r\n");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}
