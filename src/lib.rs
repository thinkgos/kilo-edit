pub mod input;
pub mod keyboard;

use keyboard::Editor;

pub fn process_key() -> Result<(), anyhow::Error> {
    let editor = Editor::new()?;

    loop {
        editor.refresh_screen()?;
        let ke = editor.read_key()?;
        if input::is_ctrl(&ke, 'q') {
            break;
        }
        print!("{e:?}\r\n", e = ke);
    }
    Ok(())
}
