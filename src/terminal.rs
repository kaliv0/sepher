use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    //TODO: terminal size
    pub height: u16, //TODO: usize?
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    //TODO: cursor position
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        // Self::clear_screen()?;
        Self::purge_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn purge_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::Purge))?;
        Ok(())
    }

    pub fn end_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::FromCursorDown))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    // pub fn print(string: &str) -> Result<(), Error> {
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        //TODO: we need Display trait implemented instead of &str -> format! with welcome_message
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        //TODO: refactor to one line -> why do we swap width and height?
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    // NB: commands are stacked for future execution -> actually run when Terminal::execute is called (i.e. flush)
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
