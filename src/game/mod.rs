pub mod computer;
pub mod player;

use crate::{Ship, ShipType, Tile};

pub fn place_ship_on_board(
    mut board: [[Tile; 10]; 10],
    ship: &Ship,
    row: usize,
    col: usize,
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
                    if board[row][i] != Tile::Unknown {
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
                    if board[i][col] != Tile::Unknown {
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
