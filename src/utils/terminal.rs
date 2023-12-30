use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;


pub fn refresh_display(lines: u16) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}