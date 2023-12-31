pub mod computer;
pub mod player;

use crate::{Ship, ShipType, Tile};

// the render bool is used to allow seeing placing ship/selector when it is over a non empty tile
pub fn place_ship_on_board(
    mut board: [[Tile; 10]; 10],
    ship: &Ship,
    row: usize,
    col: usize,
    render: bool,
) -> (bool, [[Tile; 10]; 10]) {
    match ship.ship_type {
        ShipType::CarrierHorizontal
        | ShipType::BattleshipHorizontal
        | ShipType::CruiserHorizontal
        | ShipType::SubmarineHorizontal
        | ShipType::DestroyerHorizontal => {
            if col + ship.length as usize <= 10 {
                let mut valid = true;
                for i in col..col + ship.length as usize {
                    if board[row][i] != Tile::Unknown && !render{
                        valid = false;
                    }
                }

                if valid {
                    for i in col..col + ship.length as usize {
                        board[row][i] = Tile::Ship(ship.ship_type);
                    }
                    (true, board)
                } else {
                    (false, board)
                }
            } else {
                (false, board)
            }
        }
        _ => {
            if row + ship.length as usize <= 10 {
                let mut valid = true;
                for i in row..row + ship.length as usize {
                    if board[i][col] != Tile::Unknown && !render {
                        valid = false;
                    }
                }

                if valid {
                    for i in row..row + ship.length as usize {
                        board[i][col] = Tile::Ship(ship.ship_type);
                    }
                    (true, board)
                } else {
                    (false, board)
                }
            } else {
                (false, board)
            }
        }
    }
}
