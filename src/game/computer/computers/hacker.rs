use crate::game::computer::AttackStrategy;
use crate::game::GameBoard;
use crate::{Position, Tile};

pub struct Hacker;

impl AttackStrategy for Hacker {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        for (y_coordinate, row) in enemy_board.board.iter().enumerate() {
            for (x_coordinate, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Ship(_) => {
                        return Position::new(y_coordinate as i8, x_coordinate as i8);
                    }
                    _ => continue,
                }
            }
        }

        Position::new(0, 0)
    }
}
