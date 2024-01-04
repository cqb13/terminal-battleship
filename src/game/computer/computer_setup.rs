use crate::game::place_ship_on_board;

use rand::Rng;

use crate::utils::{random_50_50, ships::get_ship};
use crate::{GameBoard, Ship, ShipType, GRID_SIZE};

pub fn computer_setup() -> GameBoard {
    let mut board = GameBoard::new();
    let ships = make_random_list_of_ships();

    let mut rng = rand::thread_rng();

    for ship in ships.iter() {
        let mut placed = false;
        while !placed {
            let row: usize = rng.gen_range(0..GRID_SIZE as usize);
            let col: usize = rng.gen_range(0..GRID_SIZE as usize);

            let result = place_ship_on_board(board.board.clone(), ship, row, col, false);

            if result.0 {
                board.board = result.1;
                placed = true;
            }
        }
    }

    board
}

fn make_random_list_of_ships() -> Vec<Ship> {
    let mut ships = Vec::new();

    if random_50_50() {
        ships.push(get_ship(ShipType::CarrierHorizontal));
    } else {
        ships.push(get_ship(ShipType::CarrierVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::BattleshipHorizontal));
    } else {
        ships.push(get_ship(ShipType::BattleshipVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::CruiserHorizontal));
    } else {
        ships.push(get_ship(ShipType::CruiserVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::SubmarineHorizontal));
    } else {
        ships.push(get_ship(ShipType::SubmarineVertical));
    }

    if random_50_50() {
        ships.push(get_ship(ShipType::DestroyerHorizontal));
    } else {
        ships.push(get_ship(ShipType::DestroyerVertical));
    }

    ships
}
