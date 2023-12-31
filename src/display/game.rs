use crate::{GameBoard, Tile, GRID_SIZE};

pub fn display_game_board(game_board: GameBoard, playing: bool) {
    let mut rows = Vec::new();
    for row in game_board.board.iter() {
        let row_string = build_row_display(row, playing);
        rows.push(row_string);
    }

    println!("   1  2  3  4  5  6  7  8  9  10");
    for (i, row) in rows.iter().enumerate() {
        println!("{} {}", (i as u8 + 65) as char, row);
    }
}

pub fn build_row_display(row: &[Tile; GRID_SIZE as usize], playing: bool) -> String {
    let mut row_string = String::new();

    for tile in row.iter() {
        match tile {
            Tile::Ship(_) => {
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
