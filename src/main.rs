use std::io::{self, stdout};

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

fn run() -> io::Result<()> {
    enable_raw_mode()?;

    let mut _stdout = stdout();

    execute!(
        _stdout,
        terminal::Clear(terminal::ClearType::All),
        MoveTo(0, 0)
    )?;

    loop {
        let event = read()?;

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

fn main() -> io::Result<()> {
    if let Err(e) = run() {
        println!("Error: {e:?}\r");
    }

    Ok(())
}
