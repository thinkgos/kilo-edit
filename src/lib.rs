pub mod keyboard;

pub fn process_key() -> Result<(), anyhow::Error> {
    loop {
        let ke = keyboard::editor_read_key()?;
        if keyboard::is_ctrl_q(&ke) {
            break;
        }
        print!("{e:?}\r\n", e = ke);
    }
    Ok(())
}
