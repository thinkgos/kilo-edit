use kilo_edit::editor::Editor;

fn main() -> Result<(), anyhow::Error> {
    let mut editor = Editor::new()?;
    editor.process()
}
