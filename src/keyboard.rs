use std::time::Duration;

use anyhow::anyhow;
use crossterm::event::{self, poll, Event, KeyEvent};
use crossterm::event::{KeyCode, KeyModifiers};

#[repr(u8)]
pub enum Arrow {
    Left = b'h',
    Right = b'l',
    Up = b'k',
    Down = b'j',
}

// impl From<char> for Arrow {
//     fn from(value: char) -> Self {
//         match value {
//             'h' => Self::Left,
//             'l' => Self::Right,
//             'k' => Self::Up,
//             'j' => Self::Down,
//             _ => Self::Left,
//         }
//     }
// }

impl TryFrom<char> for Arrow {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'h' => Ok(Self::Left),
            'l' => Ok(Self::Right),
            'k' => Ok(Self::Up),
            'j' => Ok(Self::Down),
            _ => Err(anyhow!("not supported arrow character")),
        }
    }
}

impl TryFrom<KeyCode> for Arrow {
    type Error = anyhow::Error;
    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Left => Ok(Self::Left),
            KeyCode::Right => Ok(Self::Right),
            KeyCode::Up => Ok(Self::Up),
            KeyCode::Down => Ok(Self::Down),
            _ => Err(anyhow!("not supported arrow KeyCode")),
        }
    }
}

pub struct Keyboard;

impl Keyboard {
    pub fn read(&self) -> Result<Event, anyhow::Error> {
        loop {
            match poll(Duration::from_millis(100)) {
                Ok(true) => {
                    return Ok(event::read()?);
                }
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
