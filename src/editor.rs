use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal;

use crate::keyboard::{self, Arrow, Keyboard};
use crate::screen::{Position, Screen};

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: Position,
    rows: Vec<String>,
}

impl Editor {
    pub fn new() -> Result<Editor, anyhow::Error> {
        let screen = Screen::new()?;

        terminal::enable_raw_mode()?;
        Ok(Self {
            screen,
            keyboard: Keyboard {},
            cursor: Position::default(),
            rows: vec!["hello world!".to_owned()],
        })
    }
    pub fn refresh_screen(&mut self) -> Result<(), anyhow::Error> {
        self.screen.clear()?;
        self.screen.draw_row(&self.rows)?;
        self.screen.move_cursor(&self.cursor)?;
        self.screen.flush()?;
        Ok(())
    }
    fn move_cursor(&mut self, key: Arrow) {
        let position = self.screen.bounds();
        match key {
            Arrow::Left => self.cursor.x = self.cursor.x.saturating_sub(1),
            Arrow::Right => {
                self.cursor.x += if self.cursor.x < (position.x - 1) {
                    1
                } else {
                    0
                }
            }
            Arrow::Up => self.cursor.y = self.cursor.y.saturating_sub(1),
            Arrow::Down => {
                self.cursor.y += if self.cursor.y < (position.y - 1) {
                    1
                } else {
                    0
                }
            }
            Arrow::Home => self.cursor.y = 0,
            Arrow::End => self.cursor.y = position.y - 1,
            Arrow::PageUp => self.cursor.x = 0,
            Arrow::PageDown => self.cursor.x = position.x - 1,
        }
    }

    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        loop {
            self.refresh_screen()?;

            if let Event::Key(ke) = self.keyboard.read()? {
                match ke {
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => match c {
                        'h' | 'l' | 'k' | 'j' => self.move_cursor(c.try_into()?),
                        'q' if keyboard::is_ctrl(&ke, 'q') => {
                            break;
                        }
                        _ => {}
                    },
                    KeyEvent { code, .. } => match code {
                        KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Home
                        | KeyCode::End
                        | KeyCode::PageUp
                        | KeyCode::PageDown => self.move_cursor(code.try_into()?),
                        _ => {}
                    },
                }
            }
        }
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let mut cleanup = || -> Result<(), anyhow::Error> {
            self.screen.clear()?;
            self.screen.flush()?;
            terminal::disable_raw_mode()?;
            Ok(())
        };
        cleanup().expect("editor: drop clear failed");
    }
}
