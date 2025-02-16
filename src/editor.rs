use crate::terminal::{Position, Size, Terminal};
use crate::view::View;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::cmp::min;
use std::env;
use std::io::Error;

/// location of the cursor within the text
#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        // TODO: move error handling inside main?
        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(event)?;
        }
        Ok(())
    }

    // NB: we don't pass event as ref to spare ourselves the headache in pattern matching -> TODO: refactor with ref
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                // kind: KeyEventKind::Press, //NB: for Windows only
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_point(code)?;
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                // TODO: probably we should get rid of u16 precocious re-casting? -> similar in Terminal::size()
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size { height, width });
            }
            _ => {}
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
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
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            // TODO: move logic inside terminate??
            // https://stackoverflow.com/questions/78174550/crossterm-not-clearing-screen-properly
            Terminal::clear_screen()?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
