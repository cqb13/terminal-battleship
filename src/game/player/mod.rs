pub mod player_setup;

use crate::display::game::display_game_board;
use crate::game::process_attack;
use crate::utils::terminal::{move_selector_position, refresh_display, Movement};
use crate::{GameBoard, Position, Tile};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

pub struct PlayerTurnResult {
    pub defender_board: GameBoard,
    pub valid_attack: bool,
    pub tile_at_attack: Tile,
    pub sunk_a_ship: bool,
    pub won_the_game: bool,
}

impl PlayerTurnResult {
    pub fn new(
        defender_board: GameBoard,
        valid_attack: bool,
        tile_at_attack: Tile,
        sunk_a_ship: bool,
        won_the_game: bool,
    ) -> PlayerTurnResult {
        PlayerTurnResult {
            defender_board,
            valid_attack,
            tile_at_attack,
            sunk_a_ship,
            won_the_game,
        }
    }
}

pub fn player_turn(
    mut defender_board: GameBoard,
    other_player: &String,
    attacker_board: GameBoard,
    refresh_amount: usize,
) -> PlayerTurnResult {
    let mut selector_position = Position::new(4, 4);
    let mut turn_feedback =
        PlayerTurnResult::new(defender_board, false, Tile::Unknown, false, false);

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
                    KeyCode::Down => move_selector_position(selector_position, Movement::Down, 0),
                    KeyCode::Left => move_selector_position(selector_position, Movement::Left, 0),
                    KeyCode::Right => move_selector_position(selector_position, Movement::Right, 0),
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

                            turn_feedback.tile_at_attack = feedback.tile_at_attack;

                            if feedback.sunk_a_ship {
                                turn_feedback.sunk_a_ship = true;
                            }

                            if feedback.won_the_game {
                                turn_feedback.won_the_game = true;
                            }

                            turn_feedback.valid_attack = true;

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

    turn_feedback.defender_board = defender_board;

    turn_feedback
}
