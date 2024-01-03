pub mod computer_setup;

use crate::game::process_attack;
use crate::game::GameBoard;
use crate::{Position, ShipOrientation, Tile};
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
                let adjacent_positions = get_adjacent_positions(previous_position);

                for adjacent_position in adjacent_positions {
                    if adjacent_position.is_on_board() {
                        match enemy_board.get_tile_at_position(adjacent_position) {
                            Tile::Unknown => {
                                return adjacent_position;
                            }
                            Tile::Ship(ship) => {
                                self.previous_attack_hits.push(adjacent_position);
                                if self
                                    .simulate_attack_result(enemy_board.clone(), adjacent_position)
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

    fn position_is_surrounded_by_sunk_ships(&self, enemy_board: &GameBoard, position: Position) -> bool {
        let adjacent_positions = get_adjacent_positions(position);

        for adjacent_position in adjacent_positions {
            if adjacent_position.is_on_board() {
                match enemy_board.get_tile_at_position(adjacent_position) {
                    Tile::Unknown | Tile::Ship(_) => return false,
                    _ => (),
                }
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

    fn calculate_probability(&self, enemy_board: &GameBoard, position: Position) -> f64 {
        let mut probability = 1.0;

        if enemy_board.get_tile_at_position(position) == Tile::Hit || enemy_board.get_tile_at_position(position) == Tile::Miss {
            return 0.0;
        }

        // if the position is closer to the center, increase probability by 1, up to a maximum of 5
        let center_position = Position::new(4, 4);

        let distance_from_center = (position.get_x() - center_position.get_x()).abs()
            + (position.get_y() - center_position.get_y()).abs();

        if distance_from_center < 5 {
            probability += 5.0 - distance_from_center as f64;
        }

        let adjacent_positions = get_adjacent_positions(position);

        for adjacent_position in adjacent_positions {
            if adjacent_position.is_on_board() {
                match enemy_board.get_tile_at_position(adjacent_position) {
                    Tile::Miss => {
                        probability -= 2.0;
                    }
                    Tile::Hit => {
                        probability += 20.0;
                    }
                    _ => (),
                }
            }
        }

        if self.position_is_surrounded_by_sunk_ships(enemy_board, position) {
            probability -= 30.0;
        }

        for &direction in &[ShipOrientation::Horizontal, ShipOrientation::Vertical] {
            let mut possible_positions = 0;
            for offset in 0..self.smallest_ship_length {
                let check_position = match direction {
                    ShipOrientation::Horizontal => {
                        Position::new(position.get_x() + offset, position.get_y())
                    }
                    ShipOrientation::Vertical => {
                        Position::new(position.get_x(), position.get_y() + offset)
                    }
                };
                if check_position.is_on_board() {
                    match enemy_board.get_tile_at_position(check_position) {
                        Tile::Unknown | Tile::Ship(_) => possible_positions += 1,
                        _ => break,
                    }
                }
            }

            // not enough space to fit smallest ship
            if possible_positions >= self.smallest_ship_length as usize - 1{
                probability += 3.0;
            }
        }

        probability
    }
}

impl AttackStrategy for ProbabilityAttackStrategy {
    fn calculate_best_attack(&mut self, enemy_board: &GameBoard) -> Position {
        let mut highest_probability_position = self.get_random_position(enemy_board);
        let mut highest_probability = 0.0;

        for x in 0..10 {
            for y in 0..10 {
                let position = Position::new(x as i8, y as i8);

                match enemy_board.get_tile_at_position(position) {
                    Tile::Unknown | Tile::Ship(_) => {
                        let probability = self.calculate_probability(enemy_board, position);

                        if probability > highest_probability {
                            highest_probability = probability;
                            highest_probability_position = position;
                        }
                    }
                    _ => (),
                }
            }
        }

        let simulated_attack_result =
            process_attack(enemy_board.clone(), highest_probability_position);

        if simulated_attack_result.sunk_a_ship {
            self.add_sunk_ship(simulated_attack_result.tile_at_attack);
            self.update_smallest_ship_on_board(simulated_attack_result.tile_at_attack);
        }

        // print build the array of probabilities into a grid to reflect the board
        let mut probability_grid = [[0.0; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                let position = Position::new(x as i8, y as i8);
                probability_grid[x][y] = self.calculate_probability(enemy_board, position);
            }
        }

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



        highest_probability_position
    }
}

fn get_adjacent_positions(position: Position) -> Vec<Position> {
    vec![
        Position::new(position.get_y() - 1, position.get_x()),
        Position::new(position.get_y() + 1, position.get_x()),
        Position::new(position.get_y(), position.get_x() - 1),
        Position::new(position.get_y(), position.get_x() + 1),
    ]
}
