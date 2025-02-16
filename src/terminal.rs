use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

/// terminal size
#[derive(Copy, Clone)]
pub struct Size {
    // TODO: rearrange to aligne with x-y / cols-rows??
    pub height: usize,
    pub width: usize,
}

/// cursor position
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        // Self::execute()?;  // already invoked in last refresh_screen

        //TODO: move Terminal::clear_screen()?, Terminal::show_cursor()? and Terminal::execute()?;
        // from refresh_screen screen
        // we should invoke same logic if program crashes -> especially disable_raw_mode
        // run terminate() in error handling inside main???
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { col: 0, row: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Self::queue_command(Clear(ClearType::Purge))?; // needed especially for ending the program
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    // Moves the cursor to the given Position. -> Will be truncated to `u16::MAX` if bigger.
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
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

    //TODO: should we borrow or move string into print?
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    // Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        // TODO: refactor and swap width-height
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
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
