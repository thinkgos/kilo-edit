use std::io::{self, Write};

use crossterm::{
    cursor::{self, MoveTo},
    style::Print,
    terminal, QueueableCommand,
};

use crate::VERSION;

#[derive(Debug, Default)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

pub struct Screen {
    pub width: u16,
    pub height: u16,
    stdout: io::Stdout,
}

impl Screen {
    pub fn new() -> Result<Self, anyhow::Error> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            width,
            height,
            stdout: io::stdout(),
        })
    }
    pub fn clear(&mut self) -> Result<(), anyhow::Error> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn draw_row(&mut self) -> Result<(), anyhow::Error> {
        let welcome = &|| -> String {
            let mut welcome = format!("Kilo editor -- version {}", VERSION);
            welcome.truncate(self.width as usize);
            welcome
        }();

        for row in 0..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~"))?;

            if row == self.height / 3 {
                // 输出欢迎
                let column = if welcome.len() < self.width as usize {
                    ((self.width as usize - welcome.len()) / 2) as u16
                } else {
                    0
                };
                self.stdout
                    .queue(MoveTo(column, row))?
                    .queue(Print(welcome))?;
            }
        }
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), anyhow::Error> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn get_cursor(&mut self) -> Result<CursorPosition, anyhow::Error> {
        let position = cursor::position()?;
        Ok(CursorPosition {
            x: position.0,
            y: position.1,
        })
    }

    pub fn move_cursor(&mut self, p: &CursorPosition) -> Result<(), anyhow::Error> {
        self.stdout.queue(cursor::MoveTo(p.x, p.y))?;
        Ok(())
    }
}
