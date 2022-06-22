use crate::{convert_piece_to_letter, Action, Direction, GameState, Piece, Square};
use itertools::Itertools;

// TODO add tests for this
pub fn convert_actions_to_move_string(game_state: GameState, actions: &[Action]) -> String {
    let mut game_state = game_state;
    let mut actions_as_string = Vec::new();
    let actions_last_idx = actions.len() - 1;

    for (i, action) in actions.iter().enumerate() {
        match action {
            Action::Move(square, direction) => {
                let piece_board = game_state.piece_board();
                let piece = &piece_board.piece_type_at_square(&square).unwrap();
                let is_p1_piece =
                    piece_board.bits_for_piece(*piece, true) & square.as_bit_board() != 0;

                actions_as_string.push(format!(
                    "{}{}{}",
                    convert_piece_to_letter(piece, is_p1_piece),
                    square,
                    direction
                ));

                let trapped_animal_square = &game_state.trapped_animal_for_action(&action);
                if let Some((square, piece, is_p1_piece)) = trapped_animal_square {
                    actions_as_string.push(format!(
                        "{}{}x",
                        convert_piece_to_letter(piece, *is_p1_piece),
                        square
                    ));
                }
            }
            Action::Place(piece) => {
                let piece_board = game_state.piece_board();
                let square = Square::from_bit_board(piece_board.placement_bit());

                actions_as_string.push(format!(
                    "{}{}",
                    convert_piece_to_letter(piece, game_state.is_p1_turn_to_move()),
                    square
                ));
            }
            Action::Pass => {}
        }

        let was_p1_move = game_state.is_p1_turn_to_move();
        game_state = game_state.take_action(&action);
        let is_p1_move = game_state.is_p1_turn_to_move();

        // Double space move strings when switching between players
        if i != actions_last_idx && is_p1_move != was_p1_move {
            actions_as_string.push("".to_string());
        }
    }

    actions_as_string.iter().join(" ")
}

// TODO add tests for this
pub fn convert_move_string_to_actions(actions_as_string: &str) -> Vec<Action> {
    let mut actions = actions_as_string
        .split(' ')
        .filter(|s| !s.contains('x'))
        .collect::<Vec<_>>();

    if actions.len() > 4 {
        actions.sort_by_key(|s| s[1..3].parse::<Square>().unwrap());

        actions
            .iter()
            .map(|s| {
                let piece = s[0..1].to_string().parse::<Piece>().unwrap();
                Action::Place(piece)
            })
            .collect::<Vec<_>>()
    } else {
        let mut actions = actions
            .iter()
            .map(|s| {
                let square = s[1..3].parse::<Square>().unwrap();
                let direction = s[3..4].parse::<Direction>().unwrap();
                Action::Move(square, direction)
            })
            .collect::<Vec<_>>();

        if actions.len() <= 3 {
            actions.push(Action::Pass);
        }

        actions
    }
}
