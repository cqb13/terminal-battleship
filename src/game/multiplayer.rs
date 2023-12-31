use crate::display::game::display_game_board;
use crate::display::inputs::Confirm;
use crate::game::player::player_setup::player_setup;
use crate::game::process_attack;
use crate::utils::terminal::{move_selector_position, refresh_display, Movement};
use crate::{Player, Position, Tile};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

pub fn multiplayer_game() {
    let player_one_board = player_setup(Player::PlayerOne);
    let player_two_board = player_setup(Player::PlayerTwo);

    let mut current_player = Player::PlayerOne;

    let mut confirm = false;
    while !confirm {
        confirm = Confirm::new()
            .set_message(
                "The game is about to begin, make sure player one has the computer".to_string(),
            )
            .ask();
    }

    let mut attacker_board = player_one_board.clone();
    let mut defender_board = player_two_board.clone();

    // the 4 is for the lines of letters at the top of boards, and board labels
    let mut refresh_amount = defender_board.board.len() + attacker_board.board.len() + 4;

    loop {
        let other_player = current_player.get_other_player().get_player_name();
        let mut selector_position = Position::new(4, 4);

        loop {
            let tile_to_place = match defender_board.get_tile_at_position(selector_position) {
                Tile::Ship(_) | Tile::Unknown | Tile::Targeted => Tile::Targeted,
                Tile::Hit => Tile::Hit,
                Tile::Miss => Tile::Miss,
            };

            let mut defender_board_with_selector = defender_board.clone();
            defender_board_with_selector.place_marker_on_board(selector_position, tile_to_place);
            println!("{}'s board", other_player);
            display_game_board(defender_board_with_selector, true);
            println!("Your board");
            display_game_board(attacker_board, false);

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
                                println!("{}'s board", other_player);
                                display_game_board(defender_board, true);
                                println!("Your board");
                                display_game_board(attacker_board, false);

                                if feedback.sunk_a_ship {
                                    let sunk_ship_type = match feedback.tile_at_attack {
                                        Tile::Ship(ship) => ship.get_ship_type_name(),
                                        _ => {
                                            panic!("sunk ship trigger on a tile that is not a ship")
                                        }
                                    };

                                    refresh_amount += 3;

                                    println!("");
                                    println!("You sunk {}'s {}!", other_player, sunk_ship_type);
                                    println!("");
                                }

                                if feedback.won_the_game {
                                    println!("{} won the game!", current_player.get_player_name());
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

        confirm = false;
        while !confirm {
            confirm = Confirm::new()
                .set_message(format!(
                    "Player {} are you ready to end your turn?",
                    current_player.get_player_name()
                ))
                .ask();
        }

        refresh_display(refresh_amount as u16);
        println!("{}'s board", other_player);
        display_game_board(defender_board, true);
        println!("Your board");
        display_game_board(attacker_board, true);

        confirm = false;
        while !confirm {
            confirm = Confirm::new()
                .set_message(format!(
                    "Player {} are you ready to start your turn?",
                    other_player
                ))
                .ask();
        }

        refresh_amount = defender_board.board.len() + attacker_board.board.len() + 4;

        refresh_display(refresh_amount as u16);

        current_player = current_player.get_other_player();
        let temp = attacker_board;
        attacker_board = defender_board;
        defender_board = temp;
    }
}
