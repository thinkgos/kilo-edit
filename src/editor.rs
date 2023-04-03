use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal;

use crate::keyboard::{self, Arrow, Keyboard};
use crate::screen::{CursorPosition, Screen};

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
    cursor: CursorPosition,
}

impl Editor {
    pub fn new() -> Result<Editor, anyhow::Error> {
        let screen = Screen::new()?;

        terminal::enable_raw_mode()?;
        Ok(Self {
            screen,
            keyboard: Keyboard {},
            cursor: CursorPosition::default(),
        })
    }
    pub fn refresh_screen(&mut self) -> Result<(), anyhow::Error> {
        self.screen.clear()?;
        self.screen.draw_row()?;
        self.screen.move_cursor(&self.cursor)?;
        self.screen.flush()?;
        Ok(())
    }
    fn move_cursor(&mut self, key: Arrow) {
        match key {
            Arrow::Left => self.cursor.x = self.cursor.x.saturating_sub(1),
            Arrow::Right => {
                self.cursor.x += if self.cursor.x + 1 < self.screen.width {
                    1
                } else {
                    0
                }
            }
            Arrow::Up => self.cursor.y = self.cursor.y.saturating_sub(1),
            Arrow::Down => {
                self.cursor.y += if self.cursor.y + 1 < self.screen.height {
                    1
                } else {
                    0
                }
            }
        }
    }

    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        loop {
            self.refresh_screen()?;

            match self.keyboard.read()? {
                Event::Key(ke) => match ke {
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: _,
                        kind: _,
                        state: _,
                    } => match c {
                        'h' | 'l' | 'k' | 'j' => self.move_cursor(c.try_into()?),
                        _ if keyboard::is_ctrl(&ke, 'q') => {
                            break;
                        }
                        _ => {}
                    },
                    KeyEvent { code, .. }
                        if code == KeyCode::Left
                            || code == KeyCode::Right
                            || code == KeyCode::Up
                            || code == KeyCode::Down =>
                    {
                        self.move_cursor(code.try_into()?)
                    }
                    _ => {}
                },
                _ => {}
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
