pub mod display;
pub mod game;
pub mod utils;

use display::{display_welcome, game_options};
use game::{singleplayer::singleplayer_game, multiplayer::multiplayer_game};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Targeted,
    AlreadyAttacked,
    Hit,
    Miss,
    Unknown,
    Ship(ShipType),
}

impl Tile {
    pub fn get_tile_display(&self) -> String {
        match self {
            Tile::Targeted => " ⦿ ".to_string(),
            Tile::AlreadyAttacked => " ⦿ ".to_string(),
            Tile::Hit => " 🅇 ".to_string(),
            Tile::Miss => " ⓪ ".to_string(),
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
        Self {
            ship_type,
            orientation,
            length,
        }
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

    pub fn check_if_hit_is_a_sink(self, tile_at_attack_position: Tile) -> bool {
        let mut count_of_tile_type_on_board = 0;

        for row in self.board {
            for tile in row {
                if tile == tile_at_attack_position {
                    count_of_tile_type_on_board += 1
                }

                if count_of_tile_type_on_board > 1 {
                    return false;
                }
            }
        }

        true
    }

    pub fn check_if_hit_won_the_game(self) -> bool {
        for row in self.board {
            for tile in row {
                if tile != Tile::Hit || tile != Tile::Miss || tile != Tile::Unknown {
                    return false;
                }
            }
        }

        true
    }

    pub fn set(game_board: [[Tile; 10]; 10]) -> Self {
        Self { board: game_board }
    }

    pub fn place_marker_on_board(&mut self, position: Position, tile: Tile) {
        self.board[position.get_y() as usize][position.get_x() as usize] = tile;
    }

    pub fn get_tile_at_position(&self, position: Position) -> Tile {
        self.board[position.get_y() as usize][position.get_x() as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn is_on_board(&self) -> bool {
        self.y >= 0 && self.y <= 9 && self.x >= 0 && self.x <= 9
    }
}

pub enum Player {
    PlayerOne,
    PlayerTwo,
}

impl Player {
    pub fn get_player_name(&self) -> String {
        match self {
            Player::PlayerOne => "Player One".to_string(),
            Player::PlayerTwo => "Player Two".to_string(),
        }
    }

    pub fn get_other_player(&self) -> Player {
        match self {
            Player::PlayerOne => Player::PlayerTwo,
            Player::PlayerTwo => Player::PlayerOne,
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}

pub struct GameConfig {
    game_mode: GameMode,
    difficulty: Difficulty,
}

impl GameConfig {
    pub fn new(game_mode: GameMode, difficulty: Difficulty) -> Self {
        Self {
            game_mode,
            difficulty,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }
}

fn main() {
    display_welcome();
    let config = game_options();

    match config.game_mode {
        GameMode::SinglePlayer => {
            singleplayer_game(config.difficulty);
        }
        GameMode::MultiPlayer => {
            multiplayer_game();
        }
    }
}
