use sepher::editor::Editor;

fn main() {
    Editor::new().unwrap(); //.run();

    // if let Err(err) = Editor::default().run() {
    //     eprintln!("{err}");
    //     std::process::exit(1);
    // }
}
