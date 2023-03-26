pub mod input;
pub mod keyboard;

pub fn process_key() -> Result<(), anyhow::Error> {
    loop {
        keyboard::editor_refresh_screen()?;
        let ke = keyboard::editor_read_key()?;
        if input::is_ctrl(&ke, 'q') {
            break;
        }
        print!("{e:?}\r\n", e = ke);
    }
    Ok(())
}
