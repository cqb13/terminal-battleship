pub mod display;

use display::{display_welcome, game::display_game_board, game_options};

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Hit,
    Miss,
    Unknown,
    Ship(ShipType),
}

impl Tile {
    pub fn get_tile_display(&self) -> String {
        match self {
            Tile::Hit => " 🅇 ".to_string(),
            Tile::Miss => " X ".to_string(),
            Tile::Unknown => " • ".to_string(),
            Tile::Ship(ship_type) => match ship_type {
                ShipType::CarrierHorizontal => " ▧ ".to_string(),
                ShipType::BattleshipHorizontal => " # ".to_string(),
                ShipType::CruiserHorizontal => " ▭ ".to_string(),
                ShipType::SubmarineHorizontal => " ▭ ".to_string(),
                ShipType::DestroyerHorizontal => " △ ".to_string(),
                ShipType::CarrierVertical => " ▧ ".to_string(),
                ShipType::BattleshipVertical => " # ".to_string(),
                ShipType::CruiserVertical => " ▯ ".to_string(),
                ShipType::SubmarineVertical => " ▯ ".to_string(),
                ShipType::DestroyerVertical => " △ ".to_string(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShipType {
    CarrierHorizontal,
    BattleshipHorizontal,
    CruiserHorizontal,
    SubmarineHorizontal,
    DestroyerHorizontal,
    CarrierVertical,
    BattleshipVertical,
    CruiserVertical,
    SubmarineVertical,
    DestroyerVertical,
}

pub struct Ship {
    ship_type: ShipType,
    length: u8,
}

impl Ship {
    pub fn new(ship_type: ShipType, length: u8) -> Self {
        Self { ship_type, length }
    }
}

pub struct GameBoard;

impl GameBoard {
    pub fn new() -> [[Tile; 10]; 10] {
        [[Tile::Unknown; 10]; 10]
    }
}

fn main() {
    //display_welcome();
    //let config = game_options();

    let computer_board = computer_setup();
    let player_board = GameBoard::new();

    display_game_board(&computer_board, false);
}

fn computer_setup() -> [[Tile; 10]; 10] {
    let mut board = GameBoard::new();
    let ships = make_random_list_of_ships();

    let mut rng = rand::thread_rng();

    for ship in ships.iter() {
        let mut placed = false;
        while !placed {
            let row: usize = rng.gen_range(0..10);
            let col: usize = rng.gen_range(0..10);

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
                            placed = true;
                        }
                    }
                }
                ShipType::CarrierVertical
                | ShipType::BattleshipVertical
                | ShipType::CruiserVertical
                | ShipType::SubmarineVertical
                | ShipType::DestroyerVertical => {
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
                            placed = true;
                        }
                    }
                }
            }
        }
    }

    board
}

fn make_random_list_of_ships() -> Vec<Ship> {
    let mut ships = Vec::new();

    if random_50_50() {
        ships.push(get_ship(ShipType::CarrierHorizontal));
    } else {
        ships.push(get_ship(ShipType::CarrierVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::BattleshipHorizontal));
    } else {
        ships.push(get_ship(ShipType::BattleshipVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::CruiserHorizontal));
    } else {
        ships.push(get_ship(ShipType::CruiserVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::SubmarineHorizontal));
    } else {
        ships.push(get_ship(ShipType::SubmarineVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::DestroyerHorizontal));
    } else {
        ships.push(get_ship(ShipType::DestroyerVertical));
    }

    ships
}

fn random_50_50() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}

fn get_ship(ship: ShipType) -> Ship {
    match ship {
        ShipType::CarrierHorizontal => Ship::new(ShipType::CarrierHorizontal, 5),
        ShipType::BattleshipHorizontal => Ship::new(ShipType::BattleshipHorizontal, 4),
        ShipType::CruiserHorizontal => Ship::new(ShipType::CruiserHorizontal, 3),
        ShipType::SubmarineHorizontal => Ship::new(ShipType::SubmarineHorizontal, 3),
        ShipType::DestroyerHorizontal => Ship::new(ShipType::DestroyerHorizontal, 2),
        ShipType::CarrierVertical => Ship::new(ShipType::CarrierVertical, 5),
        ShipType::BattleshipVertical => Ship::new(ShipType::BattleshipVertical, 4),
        ShipType::CruiserVertical => Ship::new(ShipType::CruiserVertical, 3),
        ShipType::SubmarineVertical => Ship::new(ShipType::SubmarineVertical, 3),
        ShipType::DestroyerVertical => Ship::new(ShipType::DestroyerVertical, 2),
    }
}
