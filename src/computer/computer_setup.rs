use rand::Rng;

use crate::utils::{random_50_50, ships::get_ship};
use crate::{GameBoard, Ship, ShipType, Tile};

pub fn computer_setup() -> [[Tile; 10]; 10] {
    let mut board = GameBoard::new();
    let ships = make_random_list_of_ships();

    let mut rng = rand::thread_rng();

    for ship in ships.iter() {
        let mut placed = false;
        while !placed {
            let row: usize = rng.gen_range(0..10);
            let col: usize = rng.gen_range(0..10);

            match ship.ship_type {
                ShipType::CarrierHorizontal
                | ShipType::BattleshipHorizontal
                | ShipType::CruiserHorizontal
                | ShipType::SubmarineHorizontal
                | ShipType::DestroyerHorizontal => {
                    if col + ship.length as usize <= 10 {
                        let mut valid = true;
                        for i in col..col + ship.length as usize {
                            if board[row][i] != Tile::Unknown {
                                valid = false;
                            }
                        }

                        if valid {
                            for i in col..col + ship.length as usize {
                                board[row][i] = Tile::Ship(ship.ship_type);
                            }
                            placed = true;
                        }
                    }
                }
                ShipType::CarrierVertical
                | ShipType::BattleshipVertical
                | ShipType::CruiserVertical
                | ShipType::SubmarineVertical
                | ShipType::DestroyerVertical => {
                    if row + ship.length as usize <= 10 {
                        let mut valid = true;
                        for i in row..row + ship.length as usize {
                            if board[i][col] != Tile::Unknown {
                                valid = false;
                            }
                        }

                        if valid {
                            for i in row..row + ship.length as usize {
                                board[i][col] = Tile::Ship(ship.ship_type);
                            }
                            placed = true;
                        }
                    }
                }
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
