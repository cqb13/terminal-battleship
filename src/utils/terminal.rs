use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

use crate::Position;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub fn refresh_display(lines: u16) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}

// cycle offset ensures that ships always stay on screen, as movement is calculated from one end of the ship only
// cycle offset should only be used when moving origin of object that is multiple tiles long, such as a ship
// cycle offset should be 0 when moving a single tile
pub fn move_selector_position(
    mut current_pos: Position,
    movement_direction: Movement,
    cycle_offset: i8,
) -> Position {
    let y = current_pos.get_y();
    let x = current_pos.get_x();

    match movement_direction {
        Movement::Up if y > 0 => current_pos.set_y(y - 1),
        Movement::Up => current_pos.set_y(9 - cycle_offset),
        Movement::Down if y + cycle_offset < 9 => current_pos.set_y(y + 1),
        Movement::Down => current_pos.set_y(0),
        Movement::Left if x > 0 => current_pos.set_x(x - 1),
        Movement::Left => current_pos.set_x(9 - cycle_offset),
        Movement::Right if x + cycle_offset < 9 => current_pos.set_x(x + 1),
        Movement::Right => current_pos.set_x(0),
    }

    current_pos
}
