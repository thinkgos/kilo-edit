use std::time::Duration;

use anyhow::anyhow;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn editor_read_key() -> Result<KeyEvent, anyhow::Error> {
    loop {
        match poll(Duration::from_millis(100)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(ke)) => {
                    return Ok(ke);
                }
                Err(_) => return Err(anyhow!("read failed!")),
                _ => {}
            },
            Ok(false) => {}
            _ => return Err(anyhow!("poll failed!")),
        }
    }
}

pub fn is_ctrl_q(ke: &KeyEvent) -> bool {
    ke.code == KeyCode::Char('q') && ke.modifiers.contains(KeyModifiers::CONTROL)
}
