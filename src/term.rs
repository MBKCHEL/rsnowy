use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::time::Duration;
use crate::snow::main_loop;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(cursor::Hide)?;
        stdout.execute(Clear(ClearType::All))?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        let _ = stdout.execute(cursor::Show);
        let _ = terminal::disable_raw_mode();
        let _ = stdout.execute(Clear(ClearType::All));
    }
}

pub fn run() -> io::Result<()> {
    let _guard = TerminalGuard::new()?;
    let mut stdout = io::stdout();

    loop {
        stdout.execute(cursor::MoveTo(0, 0))?;
        let snow = main_loop()?;
        write!(stdout, "{:#?}", snow)?;

        stdout.flush()?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('й') {
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}