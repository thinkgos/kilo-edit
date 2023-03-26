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

pub fn clear_screen(stdout: &mut io::Stdout) -> Result<(), anyhow::Error> {
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

pub fn editor_draw_row(stdio: &mut io::Stdout) -> Result<(), anyhow::Error> {
    for row in 0..24 {
        stdio.queue(cursor::MoveTo(0, row))?.queue(Print("~"))?;
    }
    Ok(())
}

pub fn editor_refresh_screen() -> Result<(), anyhow::Error> {
    let mut stdout = io::stdout();

    clear_screen(&mut stdout)?;
    editor_draw_row(&mut stdout)?;
    stdout.flush()?;
    Ok(())
}
