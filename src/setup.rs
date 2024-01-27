use crate::display::inputs::{NumberInput, OptionSelect};
use crate::{ComputerAttackStrategy, Difficulty, GameConfig, GameMode, SimulationConfig};

pub fn display_setup() {
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
    let mut config = GameConfig::new(GameMode::MultiPlayer, None, None);

    let game_mode = OptionSelect::new()
        .set_title("Game Options")
        .add_option("Play against a friend")
        .add_option("Play against the computer")
        .add_option("Computer fight")
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
        "Computer fight" => {
            config.set_game_mode(GameMode::ComputerFight);
            let attack_strategy_one = computer_options();
            let attack_strategy_two = computer_options();

            let games_to_play = NumberInput::new()
                .set_message("How many games should be played?")
                .set_min(1)
                .ask();

            let simulation_config = SimulationConfig::new(
                match_computer_option_to_computer(attack_strategy_one),
                match_computer_option_to_computer(attack_strategy_two),
                games_to_play,
            );

            config.set_simulation_config(simulation_config);
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
        .set_title("Difficulty Options")
        .add_option("Easy")
        .add_option("Medium")
        .add_option("Hard")
        .ask();

    option
}

fn computer_options() -> String {
    let option = OptionSelect::new()
        .set_title("Select a Computer Attack Strategy")
        .add_option("Random Attack")
        .add_option("Hunt and Target")
        .add_option("Probability Attack")
        .ask();

    option
}

fn match_computer_option_to_computer(strategy: String) -> ComputerAttackStrategy {
    match strategy.as_str() {
        "Random Attack" => ComputerAttackStrategy::Random,
        "Hunt and Target" => ComputerAttackStrategy::HuntAndTarget,
        "Probability Attack" => ComputerAttackStrategy::Probability,
        _ => {
            panic!("Invalid computer attack strategy selected");
        }
    }
}
