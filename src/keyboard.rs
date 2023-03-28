use std::time::Duration;

use anyhow::anyhow;
use crossterm::event::{self, poll, Event, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};

pub struct Keyboard;

impl Keyboard {
    pub fn read(&self) -> Result<KeyEvent, anyhow::Error> {
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
}

// 是否是 ctrl + {c}
pub fn is_ctrl(ke: &KeyEvent, c: char) -> bool {
    ke.code == KeyCode::Char(c) && ke.modifiers.contains(KeyModifiers::CONTROL)
}
