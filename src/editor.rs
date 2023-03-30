use crossterm::terminal;

use crate::keyboard::{self, Keyboard};
use crate::screen::Screen;

pub struct Editor {
    screen: Screen,
    keyboard: Keyboard,
}

impl Editor {
    pub fn new() -> Result<Editor, anyhow::Error> {
        let screen = Screen::new()?;

        terminal::enable_raw_mode()?;
        Ok(Self {
            screen,
            keyboard: Keyboard {},
        })
    }
    pub fn refresh_screen(&mut self) -> Result<(), anyhow::Error> {
        self.screen.clear()?;
        self.screen.draw_row()?;
        self.screen.flush()?;
        Ok(())
    }
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        loop {
            self.refresh_screen()?;
            let ke = self.keyboard.read()?;
            if keyboard::is_ctrl(&ke, 'q') {
                break;
            }
            print!("{e:?}\r\n", e = ke);
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
