use crossterm::{cursor, terminal, ExecutableCommand};
use std::io;

use crate::{Ship, ShipType, Tile};



pub fn display_game_board(game_board: &[[Tile; 10]; 10], playing: bool) {
    let mut rows = Vec::new();
    for row in game_board.iter() {
        let row_string = build_row_display(row, playing);
        rows.push(row_string);
    }

    println!("   1  2  3  4  5  6  7  8  9  10");
    for (i, row) in rows.iter().enumerate() {
        println!("{} {}", (i as u8 + 65) as char, row);
    }
}

pub fn refresh_display(lines: i16) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}
fn build_row_display(row: &[Tile; 10], playing: bool) -> String {
    let mut row_string = String::new();

    for tile in row.iter() {
        match tile {
            Tile::Ship(ship) => {
                if playing {
                    row_string.push_str(" • ");
                } else {
                    row_string.push_str(tile.get_tile_display().as_str());
                }
            }
            _ => row_string.push_str(tile.get_tile_display().as_str()),
        }
    }

    row_string
}
