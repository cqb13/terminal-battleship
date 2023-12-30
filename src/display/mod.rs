pub mod inputs;
pub mod game;

use self::inputs::OptionSelect;

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

pub fn display_welcome() {
    println!("==============================");
    println!("      Welcome to Battleship   ");
    println!("      Created by cqb13        ");
    println!("      GitHub: github.com/cqb13");
    println!("==============================");
    print!("\n");

    println!("Controls:");
    println!("  Move with arrows (←↑↓→)");
    println!("  Enter to select");
    println!("  'q' to quit");
    print!("\n");

    println!("Instructions:");
    println!("  Use arrows to navigate the board.");
    println!("  Press Enter to shoot at a tile.");
    println!("  First to sink all the ships wins!");
    print!("\n");

    println!("Enjoy the game!");
    println!("==============================");
    print!("\n");
}

pub fn game_options() -> GameConfig {
    let mut config = GameConfig::new(GameMode::MultiPlayer, Difficulty::Easy);

    let option = OptionSelect::new()
        .set_title("Game Options".to_string())
        .add_option("Play against a friend".to_string())
        .add_option("Play against the computer".to_string())
        .ask();

    print!("\n");

    match option.as_str() {
        "Play against a friend" => {}
        "Play against the computer" => {
            config.set_game_mode(GameMode::SinglePlayer);
            let difficulty = difficulty_options();
            match difficulty.as_str() {
                "Easy" => {}
                "Medium" => {
                    config.set_difficulty(Difficulty::Medium);
                }
                "Hard" => {
                    config.set_difficulty(Difficulty::Hard);
                }
                _ => {
                    panic!("Invalid difficulty selected");
                }
            }
        }
        _ => {
            panic!("Invalid game mode selected");
        }
    }

    println!("==============================");
    print!("\n");
    config
}

fn difficulty_options() -> String {
    let option = OptionSelect::new()
        .set_title("Difficulty Options".to_string())
        .add_option("Easy".to_string())
        .add_option("Medium".to_string())
        .add_option("Hard".to_string())
        .ask();

    option
}