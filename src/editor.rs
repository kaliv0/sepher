use crate::command_bar::CommandBar;
use crate::message_bar::MessageBar;
use crate::status_bar::StatusBar;
use crate::terminal::Terminal;
use crate::util::Size;
use crate::view::View;
use crossterm::event::read;
use std::env;
use std::io::Error;
use std::panic::{set_hook, take_hook};

#[derive(Eq, PartialEq, Default)]
enum PromptType {
    Search,
    Save,
    #[default]
    None,
}

impl PromptType {
    fn is_none(&self) -> bool {
        *self == PromptType::None
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: CommandBar,
    prompt_type: PromptType,
    terminal_size: Size,
    title: String,
    // quit_times: u8,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            //TODO: replace with 'Terminal::terminate()?' without assignment ??
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.update_message("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            // debug_assert!(!file_name.is_empty());
            if editor.view.load(file_name).is_err() {
                editor.update_message(&format!("ERROR: Could not open file: {file_name}"));
            }
        }
        editor.refresh_status();
        Ok(editor)
    }

    //TODO: rename 'handle_resize'
    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
    }

    fn update_message(&mut self, new_message: &str) {
        self.message_bar.update_message(new_message);
    }

    fn refresh_status(&mut self) {
        let status = self.view.get_status();
        //TODO: change to "{file_name} - sepher"
        let title = status.file_name.to_string();
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            // TODO: test alternative
            // if title != self.title && Terminal::set_title(&title).is_ok() {
            self.title = title;
        }
    }

    ///////////////////////////////
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                //TODO: we need better error handling here
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        let _ = err;
                    }
                }
            }
            self.refresh_status();
        }
    }

    // TODO: perhaps we could return Result<(), Error> here and from run() too
    //  and handle errors in main -> avoid discarding union results from Terminal::hide_caret() e.g.
    //  and also remove error hook in Editor::new() (?!)
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        // TODO: extract step in separate functions -> render_bottom_bar, render_view etc
        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
        let _ = Terminal::hide_caret();
        // if self.in_prompt() {
        //     self.command_bar.render(bottom_bar_row);
        // } else {
        self.message_bar.render(bottom_bar_row);
        // }

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        // if self.terminal_size.height > 2 {
        //     self.view.render(0);
        // }
        //
        // let new_caret_pos = if self.in_prompt() {
        //     Position {
        //         row: bottom_bar_row,
        //         col: self.command_bar.caret_position_col(),
        //     }
        // } else {
        //     self.view.caret_position()
        // };
        // debug_assert!(new_caret_pos.col <= self.terminal_size.width);
        // debug_assert!(new_caret_pos.row <= self.terminal_size.height);

        // let _ = Terminal::move_caret_to(new_caret_pos);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn in_prompt(&self) -> bool {
        // TODO: simplify and combine in one fn
        !self.prompt_type.is_none()
    }
}
