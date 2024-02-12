use crate::game::computer::AttackStrategy;
use crate::game::{process_attack, GameBoard};
use crate::{Position, Tile, DEBUG, GRID_SIZE};

pub struct ProbabilityAttackStrategy {
    sunk_ships: Vec<Tile>,
    smallest_ship_length: i8,
}

impl ProbabilityAttackStrategy {
    pub fn new() -> Self {
        Self {
            sunk_ships: Vec::new(),
            smallest_ship_length: 2,
        }
    }

    fn position_is_surrounded_by_sunk_ships(
        &self,
        enemy_board: &GameBoard,
        position: Position,
    ) -> bool {
        let adjacent_positions = self.get_adjacent_positions(position);

        for adjacent_position in adjacent_positions {
            if adjacent_position.is_on_board() {
                match enemy_board.get_tile_at_position(adjacent_position) {
                    Tile::Unknown | Tile::Ship(_) => return false,
                    _ => (),
                }
            } else {
                return false;
            }
        }

        true
    }

    fn add_sunk_ship(&mut self, tile: Tile) {
        self.sunk_ships.push(tile);
    }

    fn update_smallest_ship_on_board(&mut self, sunk_ship: Tile) {
        match sunk_ship {
            Tile::Ship(ship) => {
                if ship.get_ship_length() < self.smallest_ship_length as u8 {
                    self.smallest_ship_length = ship.get_ship_length() as i8;
                }
            }
            _ => (),
        }
    }

    fn calculate_probability(
        &self,
        enemy_board: &GameBoard,
        position: Position,
        adjacent_positions: Vec<Position>,
    ) -> f64 {
        let mut probability = 1.0;

        if enemy_board.get_tile_at_position(position) == Tile::Hit
            || enemy_board.get_tile_at_position(position) == Tile::Miss
        {
            return 0.0;
        }

        // if the position is closer to the center, increase probability by 1, up to a maximum of 5
        let center_position = Position::new(4, 4);

        let distance_from_center = (position.get_x() - center_position.get_x()).abs()
            + (position.get_y() - center_position.get_y()).abs();

        if distance_from_center < 5 {
            probability += 5.0 - distance_from_center as f64;
        }

        for adjacent_position in adjacent_positions {
            if adjacent_position.is_on_board() {
                if let Tile::Miss = enemy_board.get_tile_at_position(adjacent_position) {
                    probability -= 2.0;
                } else if let Tile::Hit = enemy_board.get_tile_at_position(adjacent_position) {
                    probability += 20.0;
                }
            }
        }

        if position.get_x() + self.smallest_ship_length < GRID_SIZE as i8 {
            probability += 1.0;
        }

        if position.get_x() - self.smallest_ship_length >= 0 {
            probability += 1.0;
        }

        if position.get_y() + self.smallest_ship_length < GRID_SIZE as i8 {
            probability += 1.0;
        }

        if position.get_y() - self.smallest_ship_length >= 0 {
            probability += 1.0;
        }

        if self.position_is_surrounded_by_sunk_ships(enemy_board, position) {
            probability -= 30.0;
        }

        probability
    }
}

impl AttackStrategy for ProbabilityAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        let mut highest_probability_position = self.get_random_position(enemy_board);
        let mut highest_probability = 0.0;

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let position = Position::new(x as i8, y as i8);

                let adjacent_positions = self.get_adjacent_positions(position);

                match enemy_board.get_tile_at_position(position) {
                    Tile::Unknown | Tile::Ship(_) => {
                        let probability =
                            self.calculate_probability(enemy_board, position, adjacent_positions);

                        if probability > highest_probability {
                            highest_probability = probability;
                            highest_probability_position = position;
                        }
                    }
                    _ => (),
                }
            }
        }

        let mut probability_grid = [[0.0; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                let position = Position::new(x as i8, y as i8);
                let adjacent_positions = self.get_adjacent_positions(position);
                probability_grid[x][y] =
                    self.calculate_probability(enemy_board, position, adjacent_positions);
            }
        }

        if DEBUG {
            println!("Probability grid:");
            let mut row_strings = Vec::new();
            for row in probability_grid.iter() {
                let mut row_string = String::new();
                for probability in row.iter() {
                    row_string.push_str(&format!(" {:.2} ", probability));
                }
                row_strings.push(row_string);
            }
            for row_string in row_strings.iter() {
                println!("{}", row_string);
            }
        }

        let simulated_attack_result = process_attack(*enemy_board, highest_probability_position);

        if simulated_attack_result.sunk_a_ship {
            self.add_sunk_ship(simulated_attack_result.tile_at_attack);
            self.update_smallest_ship_on_board(simulated_attack_result.tile_at_attack);
        }

        highest_probability_position
    }
}
