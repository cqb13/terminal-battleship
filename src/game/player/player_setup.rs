use crate::display::game::build_row_display;
use crate::display::inputs::OptionSelect;
use crate::game::place_ship_on_board;
use crate::utils::{
    ships::get_ship,
    terminal::{move_selector_position, refresh_display, Movement},
};
use crate::{GameBoard, Position, Ship, ShipType, Tile};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    terminal, ExecutableCommand,
};

pub fn player_setup() -> GameBoard {
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
            .set_title("Select Ship To Place".to_string())
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

    let mut selector_position = Position::new(4, 4);

    loop {
        display_ship_placement_board(board.board, &ship, selector_position);

        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        selector_position = if let Ok(event) = read() {
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        println!("Quitting...");
                        std::process::exit(0);
                    }
                    KeyCode::Up => move_selector_position(selector_position, Movement::Up),
                    KeyCode::Down => move_selector_position(selector_position, Movement::Down),
                    KeyCode::Left => move_selector_position(selector_position, Movement::Left),
                    KeyCode::Right => move_selector_position(selector_position, Movement::Right),
                    KeyCode::Char('r') => {
                        ship.ship_type = ship.ship_type.get_opposite_ship_type();
                        selector_position
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        let (valid, new_board) = place_ship_on_board(
                            board.board,
                            &ship,
                            selector_position.get_y() as usize,
                            selector_position.get_x() as usize,
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

fn display_ship_placement_board(board: [[Tile; 10]; 10], ship: &Ship, ship_position: Position) {
    let board_with_ship = place_ship_on_board(
        board,
        ship,
        ship_position.get_y() as usize,
        ship_position.get_x() as usize,
    );

    let mut rows = Vec::new();
    for row in board_with_ship.1.iter() {
        let row_string = build_row_display(row, false);
        rows.push(row_string);
    }

    println!("   1  2  3  4  5  6  7  8  9  10");
    for (i, row) in rows.iter().enumerate() {
        println!("{} {}", (i as u8 + 65) as char, row);
    }
}
