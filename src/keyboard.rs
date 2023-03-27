use std::{
    io::{self, Write},
    time::Duration,
};

use anyhow::anyhow;
use crossterm::{
    cursor,
    event::{self, poll, Event, KeyEvent},
    style::Print,
    terminal, QueueableCommand,
};

pub struct Editor {
    pub width: u16,
    pub height: u16,
}

impl Editor {
    pub fn new() -> Result<Editor, anyhow::Error> {
        terminal::enable_raw_mode()?;
        let (width, height) = terminal::size()?;
        Ok(Self { width, height })
    }

    pub fn read_key(&self) -> Result<KeyEvent, anyhow::Error> {
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

    pub fn clear_screen(&self, stdout: &mut io::Stdout) -> Result<(), anyhow::Error> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub fn draw_row(&self, stdio: &mut io::Stdout) -> Result<(), anyhow::Error> {
        for row in 0..24 {
            stdio.queue(cursor::MoveTo(0, row))?.queue(Print("~"))?;
        }
        Ok(())
    }

    pub fn refresh_screen(&self) -> Result<(), anyhow::Error> {
        let mut stdout = io::stdout();

        self.clear_screen(&mut stdout)?;
        self.draw_row(&mut stdout)?;
        stdout.flush()?;
        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        self.clear_screen(&mut stdout).expect("clear screen failed");
        terminal::disable_raw_mode().expect("disable raw mode failed");
    }
}
