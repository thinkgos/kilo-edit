use anyhow;
use crossterm::terminal;
use kilo_edit::process_key;

fn main() -> Result<(), anyhow::Error> {
    terminal::enable_raw_mode()?;
    let result = process_key();
    terminal::disable_raw_mode()?;
    result
}
