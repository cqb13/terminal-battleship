use crate::display::game::display_game_board;
use crate::display::inputs::Confirm;
use crate::game::computer::{
    Computer, HuntAndTargetAttackStrategy, ProbabilityAttackStrategy, RandomAttackStrategy,
};
use crate::game::player::player_setup::player_setup;
use crate::game::process_attack;
use crate::utils::terminal::{move_selector_position, refresh_display, Movement};
use crate::{Difficulty, Player, Position, Tile};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

//use crate::game::computer::computer_setup::computer_setup;

pub fn singleplayer_game(difficulty: Difficulty) {
    let mut player_one_board = player_setup(Player::PlayerOne);
    let mut computer = match difficulty {
        Difficulty::Easy => Computer::new(Box::new(RandomAttackStrategy)),
        Difficulty::Medium => Computer::new(Box::new(HuntAndTargetAttackStrategy::new())),
        Difficulty::Hard => Computer::new(Box::new(ProbabilityAttackStrategy::new())),
    };
    let computer_board = computer.computer_board;

    let mut defender_board = computer_board.clone();

    loop {
        // the 4 is for the lines of numbers at the top of boards, and board labels
        let mut refresh_amount = defender_board.board.len() + player_one_board.board.len() + 4;

        let mut selector_position = Position::new(4, 4);

        loop {
            let tile_to_place = match defender_board.get_tile_at_position(selector_position) {
                Tile::Ship(_) | Tile::Unknown | Tile::Targeted | Tile::AlreadyAttacked => {
                    Tile::Targeted
                }
                Tile::Hit => Tile::AlreadyAttacked,
                Tile::Miss => Tile::AlreadyAttacked,
            };

            let mut defender_board_with_selector = defender_board.clone();
            defender_board_with_selector.place_marker_on_board(selector_position, tile_to_place);
            println!("Computer board");
            display_game_board(defender_board_with_selector, true);
            println!("Your board");
            display_game_board(player_one_board, false);

            terminal::enable_raw_mode().expect("Failed to enable raw mode");
            selector_position = if let Ok(event) = read() {
                match event {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Char('q') => {
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            println!("Quitting...");
                            std::process::exit(0);
                        }
                        KeyCode::Up => move_selector_position(selector_position, Movement::Up, 0),
                        KeyCode::Down => {
                            move_selector_position(selector_position, Movement::Down, 0)
                        }
                        KeyCode::Left => {
                            move_selector_position(selector_position, Movement::Left, 0)
                        }
                        KeyCode::Right => {
                            move_selector_position(selector_position, Movement::Right, 0)
                        }
                        KeyCode::Enter => {
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            let feedback = process_attack(defender_board, selector_position);

                            if feedback.valid_attack {
                                match feedback.tile_at_attack {
                                    Tile::Unknown => defender_board
                                        .place_marker_on_board(selector_position, Tile::Miss),
                                    Tile::Ship(_) => defender_board
                                        .place_marker_on_board(selector_position, Tile::Hit),
                                    _ => defender_board
                                        .place_marker_on_board(selector_position, Tile::Miss),
                                }

                                refresh_display(refresh_amount as u16);
                                println!("Computer board");
                                display_game_board(defender_board, true);
                                println!("Your board");
                                display_game_board(player_one_board, false);

                                if feedback.sunk_a_ship {
                                    let sunk_ship_type = match feedback.tile_at_attack {
                                        Tile::Ship(ship) => ship.get_ship_type_name(),
                                        _ => {
                                            panic!("sunk ship trigger on a tile that is not a ship")
                                        }
                                    };

                                    refresh_amount += 2;

                                    println!("");
                                    let mut confirm = false;
                                    while !confirm {
                                        confirm = Confirm::new()
                                            .set_message(format!("You sunk the computers {}! Press enter to continue", sunk_ship_type)
                                            )
                                            .ask();
                                    }
                                    println!("");
                                }

                                if feedback.won_the_game {
                                    println!("You won the game!");
                                    std::process::exit(0);
                                }

                                break;
                            } else {
                                selector_position
                            }
                        }
                        _ => selector_position,
                    },
                    _ => selector_position,
                }
            } else {
                selector_position
            };

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(refresh_amount as u16);
        }

        refresh_display(refresh_amount as u16);

        let computer_attack_position = computer
            .attack_strategy
            .calculate_best_attack(&player_one_board);

        let feedback = process_attack(player_one_board, computer_attack_position);

        if feedback.valid_attack {
            match feedback.tile_at_attack {
                Tile::Unknown => {
                    player_one_board.place_marker_on_board(computer_attack_position, Tile::Miss)
                }
                Tile::Ship(_) => {
                    player_one_board.place_marker_on_board(computer_attack_position, Tile::Hit)
                }
                _ => player_one_board.place_marker_on_board(computer_attack_position, Tile::Miss),
            }

            if feedback.sunk_a_ship {
                let sunk_ship_type = match feedback.tile_at_attack {
                    Tile::Ship(ship) => ship.get_ship_type_name(),
                    _ => panic!("sunk ship trigger on a tile that is not a ship"),
                };

                println!("");
                let mut confirm = false;
                while !confirm {
                    confirm = Confirm::new()
                        .set_message(format!(
                            "The computer sunk your {}! Press enter to continue",
                            sunk_ship_type
                        ))
                        .ask();
                }
                println!("");

                refresh_display(2)
            }

            if feedback.won_the_game {
                println!("The computer won the game!");
                std::process::exit(0);
            }
        }
    }
}
