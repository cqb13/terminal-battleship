pub mod display;
pub mod game;
pub mod setup;
pub mod utils;

use game::{multiplayer::multiplayer_game, singleplayer::singleplayer_game};
use setup::{display_welcome, game_options};

pub const GRID_SIZE: i8 = 10;
pub const GRID_ARRAY_SIZE: i8 = 9;

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
            Tile::Targeted => " ⊕ ".to_string(),
            Tile::AlreadyAttacked => " ⊗ ".to_string(),
            Tile::Hit => " 🅇 ".to_string(),
            Tile::Miss => " ⓪ ".to_string(),
            Tile::Unknown => " • ".to_string(),
            Tile::Ship(ship_type) => ship_type.get_ship_display(),
        }
    }

    pub fn get_tile_type_name(&self) -> String {
        match self {
            Tile::Targeted => "Targeted".to_string(),
            Tile::AlreadyAttacked => "AlreadyAttacked".to_string(),
            Tile::Hit => "Hit".to_string(),
            Tile::Miss => "Miss".to_string(),
            Tile::Unknown => "Unknown".to_string(),
            Tile::Ship(ship_type) => ship_type.get_ship_type_name(),
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
            ShipType::CarrierHorizontal | ShipType::CarrierVertical => " ▧ ".to_string(),
            ShipType::BattleshipHorizontal | ShipType::BattleshipVertical => " # ".to_string(),
            ShipType::CruiserHorizontal | ShipType::SubmarineHorizontal => " ▭ ".to_string(),
            ShipType::CruiserVertical | ShipType::SubmarineVertical => " ▯ ".to_string(),
            ShipType::DestroyerHorizontal | ShipType::DestroyerVertical => " △ ".to_string(),
        }
    }

    pub fn get_ship_length(&self) -> u8 {
        match self {
            ShipType::CarrierHorizontal | ShipType::CarrierVertical => 5,
            ShipType::BattleshipHorizontal | ShipType::BattleshipVertical => 4,
            ShipType::CruiserHorizontal | ShipType::CruiserVertical => 3,
            ShipType::SubmarineHorizontal | ShipType::SubmarineVertical => 3,
            ShipType::DestroyerHorizontal | ShipType::DestroyerVertical => 2,
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
            ShipType::CarrierHorizontal | ShipType::CarrierVertical => "Carrier".to_string(),
            ShipType::BattleshipHorizontal | ShipType::BattleshipVertical => {
                "Battleship".to_string()
            }
            ShipType::CruiserHorizontal | ShipType::CruiserVertical => "Cruiser".to_string(),
            ShipType::SubmarineHorizontal | ShipType::SubmarineVertical => "Submarine".to_string(),
            ShipType::DestroyerHorizontal | ShipType::DestroyerVertical => "Destroyer".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ship {
    ship_type: ShipType,
    orientation: ShipOrientation,
}

impl Ship {
    pub fn new(ship_type: ShipType, orientation: ShipOrientation) -> Self {
        Self {
            ship_type,
            orientation,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

pub type Board = [[Tile; GRID_SIZE as usize]; GRID_SIZE as usize];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameBoard {
    pub board: Board,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            board: [[Tile::Unknown; GRID_SIZE as usize]; GRID_SIZE as usize],
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

    pub fn check_if_hit_won_the_game(&self) -> bool {
        self.board.iter().all(|row| {
            row.iter()
                .all(|tile| tile == &Tile::Hit || tile == &Tile::Miss || tile == &Tile::Unknown)
        })
    }

    pub fn set(game_board: Board) -> Self {
        Self { board: game_board }
    }

    pub fn place_marker_on_board(&mut self, position: Position, tile: Tile) {
        if position.is_on_board() {
            self.board[position.get_y() as usize][position.get_x() as usize] = tile;
        }
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
    ComputerFight,
}

pub struct GameConfig {
    game_mode: GameMode,
    difficulty: Option<Difficulty>,
    simulation_config: Option<SimulationConfig>,
}

impl GameConfig {
    pub fn new(
        game_mode: GameMode,
        difficulty: Option<Difficulty>,
        simulation_config: Option<SimulationConfig>,
    ) -> Self {
        Self {
            game_mode,
            difficulty,
            simulation_config,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = Some(difficulty);
    }

    pub fn set_simulation_config(&mut self, simulation_config: SimulationConfig) {
        self.simulation_config = Some(simulation_config);
    }
}

pub enum ComputerAttackStrategy {
    Random,
    HuntAndTarget,
    Probability,
}

impl ComputerAttackStrategy {
    pub fn get_attack_strategy_name(&self) -> String {
        match self {
            ComputerAttackStrategy::Random => "Random".to_string(),
            ComputerAttackStrategy::HuntAndTarget => "Hunt and Target".to_string(),
            ComputerAttackStrategy::Probability => "Probability Attack".to_string(),
        }
    }
}

pub struct SimulationConfig {
    pub attack_strategy_one: ComputerAttackStrategy,
    pub attack_strategy_two: ComputerAttackStrategy,
    pub games_to_play: i32,
}

impl SimulationConfig {
    pub fn new(
        attack_strategy_one: ComputerAttackStrategy,
        attack_strategy_two: ComputerAttackStrategy,
        games_to_play: i32,
    ) -> Self {
        Self {
            attack_strategy_one,
            attack_strategy_two,
            games_to_play,
        }
    }
}

fn main() {
    display_welcome();
    let config = game_options();

    match config.game_mode {
        GameMode::SinglePlayer => {
            singleplayer_game(config.difficulty.unwrap_or_else(|| {
                panic!("Difficulty not set for single player game");
            }));
        }
        GameMode::MultiPlayer => {
            multiplayer_game();
        }
        GameMode::ComputerFight => {
            
        }
    }
}
