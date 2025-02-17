use sepher::editor::Editor;

fn main() {
    //TODO: remove unwrap?!
    Editor::new().unwrap(); //.run();

    // if let Err(err) = Editor::default().run() {
    //     eprintln!("{err}");
    //     std::process::exit(1);
    // }
}
