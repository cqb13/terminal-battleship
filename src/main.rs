pub mod display;
pub mod game;
pub mod utils;

use display::{display_welcome, game::display_game_board, game_options};
use game::{computer::computer_setup::computer_setup, player::player_setup::player_setup};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Highlighted,
    Targeted,
    Hit,
    Miss,
    Unknown,
    Ship(ShipType),
}

impl Tile {
    pub fn get_tile_display(&self) -> String {
        match self {
            Tile::Highlighted => " ○ ".to_string(),
            Tile::Targeted => " ⦿ ".to_string(),
            Tile::Hit => " 🅇 ".to_string(),
            Tile::Miss => " X ".to_string(),
            Tile::Unknown => " • ".to_string(),
            Tile::Ship(ship_type) => ship_type.get_ship_display(),
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

impl ShipType {
    pub fn get_ship_display(&self) -> String {
        match self {
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
        }
    }

    pub fn get_opposite_ship_type(&self) -> ShipType {
        match self {
            ShipType::CarrierHorizontal => ShipType::CarrierVertical,
            ShipType::BattleshipHorizontal => ShipType::BattleshipVertical,
            ShipType::CruiserHorizontal => ShipType::CruiserVertical,
            ShipType::SubmarineHorizontal => ShipType::SubmarineVertical,
            ShipType::DestroyerHorizontal => ShipType::DestroyerVertical,
            ShipType::CarrierVertical => ShipType::CarrierHorizontal,
            ShipType::BattleshipVertical => ShipType::BattleshipHorizontal,
            ShipType::CruiserVertical => ShipType::CruiserHorizontal,
            ShipType::SubmarineVertical => ShipType::SubmarineHorizontal,
            ShipType::DestroyerVertical => ShipType::DestroyerHorizontal,
        }
    }
}

impl ShipType {
    pub fn get_ship_type_name(&self) -> String {
        match self {
            ShipType::CarrierHorizontal => "Carrier".to_string(),
            ShipType::BattleshipHorizontal => "Battleship".to_string(),
            ShipType::CruiserHorizontal => "Cruiser".to_string(),
            ShipType::SubmarineHorizontal => "Submarine".to_string(),
            ShipType::DestroyerHorizontal => "Destroyer".to_string(),
            ShipType::CarrierVertical => "Carrier".to_string(),
            ShipType::BattleshipVertical => "Battleship".to_string(),
            ShipType::CruiserVertical => "Cruiser".to_string(),
            ShipType::SubmarineVertical => "Submarine".to_string(),
            ShipType::DestroyerVertical => "Destroyer".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ship {
    ship_type: ShipType,
    orientation: ShipOrientation,
    length: u8,
}

impl Ship {
    pub fn new(ship_type: ShipType, orientation: ShipOrientation, length: u8) -> Self {
        Self { ship_type, orientation, length }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameBoard {
    board: [[Tile; 10]; 10],
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            board: [[Tile::Unknown; 10]; 10],
        }
    }

    pub fn set(game_board: [[Tile; 10]; 10]) -> Self {
        Self { board: game_board }
    }

    pub fn place_marker_on_board(&mut self, position: Position, tile: Tile) {
        self.board[position.get_y() as usize][position.get_x() as usize] = tile;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub y: i8,
    pub x: i8,
}

impl Position {
    pub fn new(y: i8, x: i8) -> Position {
        Position { y, x }
    }

    pub fn set_y(&mut self, y: i8) {
        self.y = y;
    }

    pub fn set_x(&mut self, x: i8) {
        self.x = x;
    }

    pub fn get_y(&self) -> i8 {
        self.y
    }

    pub fn get_x(&self) -> i8 {
        self.x
    }
}

fn main() {
    //display_welcome();
    //let config = game_options();

    let computer_board = computer_setup();
    let player_board = player_setup();

    display_game_board(player_board, false);
}
