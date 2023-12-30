pub mod display;
pub mod computer;
pub mod utils;

use display::{display_welcome, game::display_game_board, game_options};
use computer::computer_setup::computer_setup;

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
