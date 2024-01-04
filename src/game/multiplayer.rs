use crate::display::{game::display_game_board, inputs::Confirm};
use crate::game::player::{player_setup::player_setup, player_turn};
use crate::utils::terminal::refresh_display;
use crate::Player;

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

    // the 4 is for the lines of numbers at the top of boards, and board labels
    let mut refresh_amount = defender_board.board.len() + attacker_board.board.len() + 4;

    loop {
        let other_player = current_player.get_other_player().get_player_name();

        let player_turn_result = player_turn(
            defender_board,
            &other_player,
            attacker_board,
            refresh_amount,
        );

        defender_board = player_turn_result.defender_board;

        if player_turn_result.sunk_a_ship {
            println!("");
            println!(
                "You sunk {}'s {}!",
                other_player,
                player_turn_result.tile_at_attack.get_tile_type_name()
            );
            println!("");

            refresh_amount += 3;
        }

        if player_turn_result.won_the_game {
            println!("{} won the game!", current_player.get_player_name());
            break;
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
