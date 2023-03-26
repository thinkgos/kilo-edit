use crossterm::terminal;

use kilo_edit::{keyboard, process_key};

fn main() -> Result<(), anyhow::Error> {
    terminal::enable_raw_mode()?;
    let result = process_key();

    let mut stdout = std::io::stdout();
    let _ = keyboard::clear_screen(&mut stdout);
    terminal::disable_raw_mode()?;
    result
}
