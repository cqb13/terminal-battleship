use crate::display::inputs::Confirm;
use crate::game::computer::{
    Computer, HuntAndTargetAttackStrategy, ProbabilityAttackStrategy, RandomAttackStrategy,
};
use crate::game::player::{player_setup::player_setup, player_turn};
use crate::game::process_attack;
use crate::utils::terminal::refresh_display;
use crate::{Difficulty, Player, Tile};

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

        let player_turn_result = player_turn(
            defender_board,
            &"Computer".to_string(),
            player_one_board,
            refresh_amount,
        );

        defender_board = player_turn_result.defender_board;

        if player_turn_result.sunk_a_ship {
            println!("");
            println!(
                "You sunk Computers {}!",
                player_turn_result.tile_at_attack.get_tile_type_name()
            );
            println!("");

            refresh_amount += 3;
        }

        if player_turn_result.won_the_game {
            println!("You won the game!");
            break;
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
                break;
            }
        }
    }
}
