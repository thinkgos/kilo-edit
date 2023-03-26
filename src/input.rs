use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// 是否是 ctrl + {c}
pub fn is_ctrl(ke: &KeyEvent, c: char) -> bool {
    ke.code == KeyCode::Char(c) && ke.modifiers.contains(KeyModifiers::CONTROL)
}
