use crate::game::computer::AttackStrategy;
use crate::game::{process_attack, GameBoard};
use crate::{Position, Tile};

pub struct HuntAndTargetAttackStrategy {
    previous_attack_hits: Vec<Position>,
}

impl HuntAndTargetAttackStrategy {
    pub fn new() -> Self {
        Self {
            previous_attack_hits: Vec::new(),
        }
    }

    fn simulate_attack_result(&mut self, enemy_board: GameBoard, position: Position) -> bool {
        let feedback = process_attack(enemy_board, position);

        feedback.sunk_a_ship
    }

    fn remove_hits_from_previous_attack_on_sink(
        &mut self,
        enemy_board: &GameBoard,
        sunk_ship_tile: Tile,
    ) {
        self.previous_attack_hits
            .retain(|&position| enemy_board.get_tile_at_position(position) != sunk_ship_tile);
    }
}

impl AttackStrategy for HuntAndTargetAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        if !self.previous_attack_hits.is_empty() {
            for previous_position in self.previous_attack_hits.clone() {
                let adjacent_positions = self.get_adjacent_positions(previous_position);
                for adjacent_position in adjacent_positions {
                    if !adjacent_position.is_on_board() {
                        continue;
                    }

                    match enemy_board.get_tile_at_position(adjacent_position) {
                        Tile::Unknown => {
                            return adjacent_position;
                        }
                        Tile::Ship(ship) => {
                            self.previous_attack_hits.push(adjacent_position);
                            if self.simulate_attack_result(enemy_board.clone(), adjacent_position) {
                                self.remove_hits_from_previous_attack_on_sink(
                                    enemy_board,
                                    Tile::Ship(ship),
                                );
                            }
                            return adjacent_position;
                        }
                        _ => (),
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
