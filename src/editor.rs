use std::io::{self, stdout, Stdout};

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

pub struct Editor {
    stdout: Stdout,
}

impl Editor {
    pub fn default() -> Self {
        Self { stdout: stdout() }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;

        execute!(
            self.stdout,
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
}
