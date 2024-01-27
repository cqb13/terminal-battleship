use crate::game::computer::computers::{
    hunt_and_target::HuntAndTargetAttackStrategy, probability_attack::ProbabilityAttackStrategy,
    random_attack::RandomAttackStrategy,
};
use crate::game::computer::Computer;
use crate::game::process_attack;
use crate::{ComputerAttackStrategy, SimulationConfig, Tile};

pub struct SimulationResults {
    pub stats: Vec<SimulationResult>,
    pub computer_one_attack_strategy: ComputerAttackStrategy,
    pub computer_two_attack_strategy: ComputerAttackStrategy,
    pub computer_one_wins: i32,
    pub computer_two_wins: i32,
    pub games_played: i32,
}

impl SimulationResults {
    pub fn new(
        computer_one_attack_strategy: ComputerAttackStrategy,
        computer_two_attack_strategy: ComputerAttackStrategy,
    ) -> Self {
        Self {
            stats: Vec::new(),
            computer_one_attack_strategy,
            computer_two_attack_strategy,
            computer_one_wins: 0,
            computer_two_wins: 0,
            games_played: 0,
        }
    }

    pub fn add_simulation_result(&mut self, simulation_result: SimulationResult) {
        match simulation_result.winner {
            ComputerPlayer::ComputerOne => self.computer_one_wins += 1,
            ComputerPlayer::ComputerTwo => self.computer_two_wins += 1,
        }

        self.stats.push(simulation_result);

        self.games_played += 1;
    }

    pub fn print_results(&self) {
        println!("");
        println!("Simulation Results");
        println!("------------------");
        println!("");

        println!(
            "Computer One Attack Strategy: {}",
            self.computer_one_attack_strategy.get_attack_strategy_name()
        );
        println!(
            "Computer Two Attack Strategy: {}",
            self.computer_two_attack_strategy.get_attack_strategy_name()
        );
        println!("");

        println!("Games Played: {}", self.games_played);
        println!("");

        println!("Computer One Wins: {}", self.computer_one_wins);
        println!("Computer Two Wins: {}", self.computer_two_wins);
        println!("");

        println!(
            "Computer One Win Percentage: {}%",
            self.get_win_percentage(ComputerPlayer::ComputerOne)
        );
        println!(
            "Computer Two Win Percentage: {}%",
            self.get_win_percentage(ComputerPlayer::ComputerTwo)
        );
        println!("");
    }

    fn get_win_percentage(&self, computer: ComputerPlayer) -> f32 {
        match computer {
            ComputerPlayer::ComputerOne => {
                (self.computer_one_wins as f32 / self.games_played as f32) * 100.0
            }
            ComputerPlayer::ComputerTwo => {
                (self.computer_two_wins as f32 / self.games_played as f32) * 100.0
            }
        }
    }
}

pub struct SimulationResult {
    pub computer_one_stats: ComputerStats,
    pub computer_two_stats: ComputerStats,
    pub winner: ComputerPlayer,
}

impl SimulationResult {
    pub fn new(
        computer_one_stats: ComputerStats,
        computer_two_stats: ComputerStats,
        winner: ComputerPlayer,
    ) -> Self {
        Self {
            computer_one_stats,
            computer_two_stats,
            winner,
        }
    }
}

#[derive(Debug)]
pub struct ComputerStats {
    pub hits: u32,
    pub misses: u32,
    pub ships_sunk: u32,
    pub shots_fired: u32,
}

pub enum ComputerPlayer {
    ComputerOne,
    ComputerTwo,
}

pub fn simulated_game(simulation_config: SimulationConfig) {
    let mut computer_one = Computer::new(match simulation_config.attack_strategy_one {
        ComputerAttackStrategy::Random => Box::new(RandomAttackStrategy),
        ComputerAttackStrategy::HuntAndTarget => Box::new(HuntAndTargetAttackStrategy::new()),
        ComputerAttackStrategy::Probability => Box::new(ProbabilityAttackStrategy::new()),
    });

    let mut computer_two = Computer::new(match simulation_config.attack_strategy_two {
        ComputerAttackStrategy::Random => Box::new(RandomAttackStrategy),
        ComputerAttackStrategy::HuntAndTarget => Box::new(HuntAndTargetAttackStrategy::new()),
        ComputerAttackStrategy::Probability => Box::new(ProbabilityAttackStrategy::new()),
    });

    let games_to_simulate = simulation_config.games_to_play;

    let mut simulation_results = SimulationResults::new(
        simulation_config.attack_strategy_one,
        simulation_config.attack_strategy_two,
    );

    let mut games_played = 0;

    println!("Simulating Games...");

    while games_played < games_to_simulate {
        computer_one.regenerate_computer_board();
        computer_two.regenerate_computer_board();

        let mut computer_one_board = computer_one.computer_board.clone();
        let mut computer_two_board = computer_two.computer_board.clone();

        let mut computer_one_stats = ComputerStats {
            hits: 0,
            misses: 0,
            ships_sunk: 0,
            shots_fired: 0,
        };

        let mut computer_two_stats = ComputerStats {
            hits: 0,
            misses: 0,
            ships_sunk: 0,
            shots_fired: 0,
        };

        loop {
            let computer_one_attack_position = computer_one
                .attack_strategy
                .calculate_best_attack(&computer_two_board);

            let feedback = process_attack(computer_two_board, computer_one_attack_position);

            if feedback.valid_attack {
                match feedback.tile_at_attack {
                    Tile::Unknown => computer_two_board
                        .place_marker_on_board(computer_one_attack_position, Tile::Miss),
                    Tile::Ship(_) => computer_two_board
                        .place_marker_on_board(computer_one_attack_position, Tile::Hit),
                    _ => computer_two_board
                        .place_marker_on_board(computer_one_attack_position, Tile::Miss),
                }
            }

            if feedback.sunk_a_ship {
                computer_one_stats.ships_sunk += 1;
            }

            computer_one_stats.shots_fired += 1;

            if feedback.hit_a_ship {
                computer_one_stats.hits += 1;
            } else {
                computer_one_stats.misses += 1;
            }

            if feedback.won_the_game {
                break;
            }

            let computer_two_attack_position = computer_two
                .attack_strategy
                .calculate_best_attack(&computer_one_board);

            let feedback = process_attack(computer_one_board, computer_two_attack_position);

            if feedback.valid_attack {
                match feedback.tile_at_attack {
                    Tile::Unknown => computer_one_board
                        .place_marker_on_board(computer_two_attack_position, Tile::Miss),
                    Tile::Ship(_) => computer_one_board
                        .place_marker_on_board(computer_two_attack_position, Tile::Hit),
                    _ => computer_one_board
                        .place_marker_on_board(computer_two_attack_position, Tile::Miss),
                }
            }

            if feedback.sunk_a_ship {
                computer_two_stats.ships_sunk += 1;
            }

            computer_two_stats.shots_fired += 1;

            if feedback.hit_a_ship {
                computer_two_stats.hits += 1;
            } else {
                computer_two_stats.misses += 1;
            }

            if feedback.won_the_game {
                break;
            }
        }

        let winner = if computer_one_stats.ships_sunk > computer_two_stats.ships_sunk {
            ComputerPlayer::ComputerOne
        } else {
            ComputerPlayer::ComputerTwo
        };

        let simulation_result =
            SimulationResult::new(computer_one_stats, computer_two_stats, winner);

        simulation_results.add_simulation_result(simulation_result);

        games_played += 1;
        show_simulation_progress(games_played, games_to_simulate);
    }

    simulation_results.print_results();
}

fn show_simulation_progress(games_played: i32, games_to_play: i32) {
    let games_played = games_played.min(games_to_play);
    let progress = (games_played as f32 / games_to_play as f32) * 100.0;

    let progress_bar_length = 20;
    let progress_bar_fill = (progress / 100.0 * progress_bar_length as f32) as usize;

    let mut progress_bar = String::from("[");

    for i in 0..progress_bar_length {
        if i < progress_bar_fill {
            progress_bar.push('=');
        } else if i == progress_bar_fill {
            progress_bar.push('>');
        } else {
            progress_bar.push(' ');
        }
    }

    progress_bar.push(']');
    progress_bar.push_str(&format!(" {:.2}%", progress));

    print!("\r{}", progress_bar);
}
