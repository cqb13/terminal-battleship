pub mod computer_setup;

pub mod computers {
    pub mod hunt_and_target;
    pub mod probability_attack;
    pub mod random_attack;
}

use crate::game::GameBoard;
use crate::{Position, Tile, GRID_SIZE};
use computer_setup::computer_setup;

//TODO: Clean up this file

pub struct Computer {
    pub computer_board: GameBoard,
    pub attack_strategy: Box<dyn AttackStrategy>,
}

impl Computer {
    pub fn new(attack_strategy: Box<dyn AttackStrategy>) -> Self {
        let computer_board = computer_setup();
        Self {
            computer_board,
            attack_strategy,
        }
    }

    pub fn regenerate_computer_board(&mut self) {
        self.computer_board = computer_setup();
    }
}

pub trait AttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position;

    fn generate_random_position(&self) -> Position {
        let x = rand::random::<usize>() % GRID_SIZE as usize;
        let y = rand::random::<usize>() % GRID_SIZE as usize;

        Position::new(x as i8, y as i8)
    }

    fn get_random_position(&mut self, enemy_board: &GameBoard) -> Position {
        let mut position = self.generate_random_position();

        loop {
            match enemy_board.get_tile_at_position(position) {
                Tile::Unknown | Tile::Ship(_) => break,
                _ => position = self.generate_random_position(),
            }
        }

        position
    }

    fn get_adjacent_positions(&self, position: Position) -> Vec<Position> {
        vec![
            Position::new(position.get_y() - 1, position.get_x()),
            Position::new(position.get_y() + 1, position.get_x()),
            Position::new(position.get_y(), position.get_x() - 1),
            Position::new(position.get_y(), position.get_x() + 1),
        ]
    }
}
