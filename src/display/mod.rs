pub mod game;
pub mod inputs;

use self::inputs::OptionSelect;
use crate::{Difficulty, GameConfig, GameMode};

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

    let game_mode = OptionSelect::new()
        .set_title("Game Options".to_string())
        .add_option("Play against a friend".to_string())
        .add_option("Play against the computer".to_string())
        .ask();

    print!("\n");

    match game_mode.as_str() {
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
