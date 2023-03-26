use crossterm::{
    event::{self, poll, Event, KeyCode, KeyModifiers},
    terminal,
};
use std::{error, process, time::Duration};

use errno::errno;

fn main() -> Result<(), Box<dyn error::Error>> {
    terminal::enable_raw_mode()?;
    let mut count = 0;
    loop {
        count += 1;

        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                match event::read() {
                    Ok(Event::Key(ke)) => {
                        // NOTE: println的换行会失效, ctrl + c也会失效
                        print!("{count} - {e:?}\r\n", count = count, e = ke);
                        if ke.code == KeyCode::Char('q')
                            && ke.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break;
                        }
                    }
                    Err(_) => die("read failed!"),
                    _ => {}
                }
            }
            Ok(false) => {}
            _ => die("poll failed!"), // TODO: 什么情况才会发生?
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn die<S: Into<String>>(s: S) {
    let _ = terminal::disable_raw_mode();
    print!("{} - {}\r\n", s.into(), errno());
    process::exit(1);
}
