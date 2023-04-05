use std::env;

use kilo_edit::editor::Editor;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    let mut editor = if args.len() >= 2 {
        Editor::with_file(&args[1])?
    } else {
        Editor::new()?
    };
    editor.process()
}
