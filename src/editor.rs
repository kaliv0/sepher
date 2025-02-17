use crate::message_bar::MessageBar;
use crate::status_bar::StatusBar;
use crate::terminal::Terminal;
use crate::ui_component::UIComponent;
use crate::util::Size;
use crate::view::View;
use std::env;
use std::io::Error;
use std::panic::{set_hook, take_hook};

#[derive(Default)]
pub struct Editor {
    // should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    // command_bar: CommandBar,
    // prompt_type: PromptType,
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
        // self.command_bar.resize(bar_size);
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
}
