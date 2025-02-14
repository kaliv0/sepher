#![warn(
    clippy::all,
    clippy::pedantic,
    // clippy::nursery,
    // clippy::cargo,
    // clippy::restriction
)]

use sepher::editor::Editor;

fn main() {
    if let Err(err) = Editor::new().run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
    print!("Goodbye.\r\n");
}
