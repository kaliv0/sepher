#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    // clippy::nursery,
    // clippy::cargo,
    // clippy::restriction
)]

use sepher::editor::Editor;

fn main() {
    Editor::default().run();

    // if let Err(err) = Editor::default().run() {
    //     eprintln!("{err}");
    //     std::process::exit(1);
    // }
    // print!("Goodbye.\r\n"); // TODO: remove
}
