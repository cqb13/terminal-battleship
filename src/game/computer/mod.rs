pub mod computer_setup;

use crate::game::process_attack;
use crate::game::GameBoard;
use crate::{Position, Tile};
use computer_setup::computer_setup;

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
}

pub trait AttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position;

    fn generate_random_position(&self) -> Position {
        let x = rand::random::<usize>() % 10;
        let y = rand::random::<usize>() % 10;

        Position::new(x as i8, y as i8)
    }

    fn get_random_position(&mut self, enemy_board: &GameBoard) -> Position {
        let mut position = self.generate_random_position();

        loop {
            match enemy_board.get_tile_at_position(position) {
                Tile::Unknown => break,
                Tile::Ship(_) => break,
                _ => position = self.generate_random_position(),
            }
        }

        position
    }
}

pub struct RandomAttackStrategy;

impl AttackStrategy for RandomAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        self.get_random_position(enemy_board)
    }
}

pub struct HuntAndTargetAttackStrategy {
    previous_attack_hits: Vec<Position>,
}

impl HuntAndTargetAttackStrategy {
    pub fn new() -> Self {
        Self {
            previous_attack_hits: Vec::new(),
        }
    }
}

impl HuntAndTargetAttackStrategy {
    pub fn remove_previous_attack_hit(&mut self, position: Position) {
        self.previous_attack_hits.retain(|&x| x != position);
    }

    pub fn add_previous_attack_hit(&mut self, position: Position) {
        self.previous_attack_hits.push(position);
    }

    pub fn simulate_attack_result(&mut self, enemy_board: GameBoard, position: Position) -> bool {
        let feedback = process_attack(enemy_board, position);

        feedback.sunk_a_ship
    }

    pub fn remove_hits_from_previous_attack_on_sink(
        &mut self,
        enemy_board: &GameBoard,
        sunk_ship_tile: Tile,
    ) {
        for position in self.previous_attack_hits.clone() {
            if enemy_board.get_tile_at_position(position) == sunk_ship_tile {
                self.remove_previous_attack_hit(position);
            }
        }
    }
}

impl AttackStrategy for HuntAndTargetAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        if self.previous_attack_hits.len() != 0 {
            for previous_position in self.previous_attack_hits.clone() {
                let adjacent_positions = vec![
                    Position::new(previous_position.get_y() - 1, previous_position.get_x()),
                    Position::new(previous_position.get_y() + 1, previous_position.get_x()),
                    Position::new(previous_position.get_y(), previous_position.get_x() - 1),
                    Position::new(previous_position.get_y(), previous_position.get_x() + 1),
                ];

                for adjacent_position in adjacent_positions {
                    if adjacent_position.is_on_board() {
                        match enemy_board.get_tile_at_position(adjacent_position) {
                            Tile::Unknown => {
                                return adjacent_position;
                            }
                            Tile::Ship(ship) => {
                                self.previous_attack_hits.push(adjacent_position);
                                if self.simulate_attack_result(enemy_board.clone(), adjacent_position)
                                {
                                    self.remove_hits_from_previous_attack_on_sink(
                                        enemy_board,
                                        Tile::Ship(ship),
                                    );

                                    println!("Sunk a ship, {:?}", self.previous_attack_hits);
                                }

                                return adjacent_position;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        let position = self.get_random_position(enemy_board);

        match enemy_board.get_tile_at_position(position) {
            Tile::Ship(_) => {
                self.previous_attack_hits.push(position);
            }
            _ => (),
        }

        return position;
    }
}
