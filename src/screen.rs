use std::io::{self, Write};

use crossterm::{cursor, style::Print, terminal, QueueableCommand};

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
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub fn draw_row(&mut self) -> Result<(), anyhow::Error> {
        for row in 0..self.height {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~"))?;
        }
        self.stdout.flush()?;
        Ok(())
    }
}
