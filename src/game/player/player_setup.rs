use crate::display::game::display_game_board;
use crate::display::inputs::OptionSelect;
use crate::game::place_ship_on_board;
use crate::utils::{
    ships::get_ship,
    terminal::{move_selector_position, refresh_display, Movement},
};
use crate::{GameBoard, Player, Position, ShipOrientation, ShipType};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

pub fn player_setup(player: Player) -> GameBoard {
    let mut board = GameBoard::new();

    let mut ship_names = [
        "Carrier".to_string(),
        "Battleship".to_string(),
        "Cruiser".to_string(),
        "Submarine".to_string(),
        "Destroyer".to_string(),
    ]
    .to_vec();

    while ship_names.len() > 0 {
        let option_template = OptionSelect::new()
            .set_title(format!(
                "{}, Select a ship to place",
                player.get_player_name()
            ))
            .add_option_if_true(
                "Carrier".to_string(),
                ship_names.contains(&"Carrier".to_string()),
            )
            .add_option_if_true(
                "Battleship".to_string(),
                ship_names.contains(&"Battleship".to_string()),
            )
            .add_option_if_true(
                "Cruiser".to_string(),
                ship_names.contains(&"Cruiser".to_string()),
            )
            .add_option_if_true(
                "Submarine".to_string(),
                ship_names.contains(&"Submarine".to_string()),
            )
            .add_option_if_true(
                "Destroyer".to_string(),
                ship_names.contains(&"Destroyer".to_string()),
            )
            .ask();

        let additional_lines = 11;

        // also clears game board
        if ship_names.len() != 5 {
            refresh_display(ship_names.len() as u16 + 1 + additional_lines);
        } else {
            refresh_display(ship_names.len() as u16 + 1);
        }

        let ship_type = match option_template.as_str() {
            "Carrier" => {
                ship_names.retain(|x| x != &"Carrier".to_string());
                ShipType::CarrierHorizontal
            }
            "Battleship" => {
                ship_names.retain(|x| x != &"Battleship".to_string());
                ShipType::BattleshipHorizontal
            }
            "Cruiser" => {
                ship_names.retain(|x| x != &"Cruiser".to_string());
                ShipType::CruiserHorizontal
            }
            "Submarine" => {
                ship_names.retain(|x| x != &"Submarine".to_string());
                ShipType::SubmarineHorizontal
            }
            "Destroyer" => {
                ship_names.retain(|x| x != &"Destroyer".to_string());
                ShipType::DestroyerHorizontal
            }
            _ => panic!("Invalid ship type"),
        };

        ship_placement_selection(&mut board, ship_type);
    }
    refresh_display(11);

    board
}

fn ship_placement_selection(board: &mut GameBoard, ship: ShipType) {
    let mut ship = get_ship(ship);
    let ship_length = ship.ship_type.get_ship_length();

    let mut selector_position = Position::new(4, 4 - calculate_ship_center(ship_length as i8));

    loop {
        let board_with_ship = GameBoard::set(
            place_ship_on_board(
                board.board,
                &ship,
                selector_position.get_y() as usize,
                selector_position.get_x() as usize,
                true,
            )
            .1,
        );

        display_game_board(board_with_ship, false);

        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        selector_position = if let Ok(event) = read() {
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        println!("Quitting...");
                        std::process::exit(0);
                    }
                    KeyCode::Up => {
                        let cycle_offset = match ship.orientation {
                            ShipOrientation::Horizontal => 0,
                            ShipOrientation::Vertical => ship_length - 1,
                        };

                        move_selector_position(selector_position, Movement::Up, cycle_offset as i8)
                    }
                    KeyCode::Down => {
                        let cycle_offset = match ship.orientation {
                            ShipOrientation::Horizontal => 0,
                            ShipOrientation::Vertical => ship_length - 1,
                        };

                        move_selector_position(
                            selector_position,
                            Movement::Down,
                            cycle_offset as i8,
                        )
                    }
                    KeyCode::Left => {
                        let cycle_offset = match ship.orientation {
                            ShipOrientation::Horizontal => ship_length - 1,
                            ShipOrientation::Vertical => 0,
                        };

                        move_selector_position(
                            selector_position,
                            Movement::Left,
                            cycle_offset as i8,
                        )
                    }
                    KeyCode::Right => {
                        let cycle_offset = match ship.orientation {
                            ShipOrientation::Horizontal => ship_length - 1,
                            ShipOrientation::Vertical => 0,
                        };

                        move_selector_position(
                            selector_position,
                            Movement::Right,
                            cycle_offset as i8,
                        )
                    }
                    KeyCode::Char('r') => {
                        let mut x = selector_position.get_x();
                        let mut y = selector_position.get_y();
                        let transform_amount = calculate_ship_center(ship_length as i8);

                        match ship.orientation {
                            ShipOrientation::Horizontal => {
                                y -= transform_amount;
                                x += transform_amount;
                            }
                            ShipOrientation::Vertical => {
                                y += transform_amount;
                                x -= transform_amount;
                            }
                        }

                        // ensure ship stays on screen
                        if x < 0 {
                            x = 0;
                        } else if x + ship_length as i8 > 10 {
                            x = 10 - ship_length as i8;
                        }

                        if y < 0 {
                            y = 0;
                        } else if y + ship_length as i8 > 10 {
                            y = 10 - ship_length as i8;
                        }

                        ship.orientation = match ship.orientation {
                            ShipOrientation::Horizontal => ShipOrientation::Vertical,
                            ShipOrientation::Vertical => ShipOrientation::Horizontal,
                        };

                        ship.ship_type = ship.ship_type.get_opposite_ship_type();

                        Position::new(y, x)
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        let (valid, new_board) = place_ship_on_board(
                            board.board,
                            &ship,
                            selector_position.get_y() as usize,
                            selector_position.get_x() as usize,
                            false,
                        );

                        if valid {
                            board.board = new_board;
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
        refresh_display(11);
    }
}

fn calculate_ship_center(ship_length: i8) -> i8 {
    if ship_length % 2 == 0 {
        ship_length / 2
    } else {
        (ship_length - 1) / 2
    }
}
