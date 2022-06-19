#[cfg(test)]
mod tests {
    use super::super::{take_actions, PushPullState, Terminal};
    use super::super::{GameState, Piece, Square};

    fn place_major_pieces(game_state: GameState) -> GameState {
        take_actions!(game_state => h, c, d, m, e, d, c, h)
    }

    fn place_8_rabbits(game_state: GameState) -> GameState {
        take_actions!(game_state => r, r, r, r, r, r, r, r)
    }

    fn initial_play_state() -> GameState {
        let game_state = GameState::initial();
        let game_state = place_8_rabbits(game_state);
        let game_state = place_major_pieces(game_state);

        let game_state = place_major_pieces(game_state);
        place_8_rabbits(game_state)
    }

    #[test]
    fn test_action_placing_pieces() {
        let game_state = initial_play_state();
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.cats,
            0b__01000010__00000000__00000000__00000000__00000000__00000000__00000000__01000010
        );
        assert_eq!(
            piece_board.dogs,
            0b__00100100__00000000__00000000__00000000__00000000__00000000__00000000__00100100
        );
        assert_eq!(
            piece_board.horses,
            0b__10000001__00000000__00000000__00000000__00000000__00000000__00000000__10000001
        );
        assert_eq!(
            piece_board.camels,
            0b__00001000__00000000__00000000__00000000__00000000__00000000__00000000__00001000
        );
        assert_eq!(
            piece_board.elephants,
            0b__00010000__00000000__00000000__00000000__00000000__00000000__00000000__00010000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );

        assert!(game_state.is_p1_turn_to_move());
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::None
        );
    }

    #[test]
    fn test_action_move_up() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => a2n);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111110__00000001__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111110__00000001__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => a3n);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111110__00000000__00000001__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111110__00000000__00000001__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => a4n);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111110__00000000__00000000__00000001__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111110__00000000__00000000__00000001__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 3);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => b2n);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111100__00000010__00000000__00000001__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111100__00000010__00000000__00000001__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_down() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => d7s);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00001000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => d6s);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00001000__00000000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => d5s);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00001000__00000000__00000000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 3);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => d4s);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00001000__00000000__00000000__00000000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_left() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => d7s);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00001000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => d6w);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00000100__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => c6w);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00000010__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 3);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => b6w);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00000001__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_right() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => d7s);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00001000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => d6e);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00010000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => e6e);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__00100000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 3);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => f6e);
        let piece_board = game_state.piece_board();
        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111111__00000000__00000000__00000000__01000000__11110111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111111__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_unsupported() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => c2n);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111011__00000000__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111011__00000000__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_supported_right() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => b2n);
        let game_state = take_actions!(game_state => c2n);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111001__00000110__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111001__00000110__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_supported_left() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => d2n);
        let game_state = take_actions!(game_state => c2n);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11110011__00001100__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11110011__00001100__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_supported_top() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => b2n);
        let game_state = take_actions!(game_state => b3e);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111101__00000100__00000000__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111101__00000100__00000000__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 2);
        assert!(game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_supported_bottom() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => b2n);
        let game_state = take_actions!(game_state => b3n);
        let game_state = take_actions!(game_state => b4e);
        let game_state = take_actions!(game_state => c2n);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111001__00000100__00000100__00000000__00000000__11111111__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111001__00000100__00000100__00000000__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_trap_adjacent_opp_unsupported() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => b2n);
        let game_state = take_actions!(game_state => c2n);
        let game_state = take_actions!(game_state => c3n);
        let game_state = take_actions!(game_state => c4n);

        let game_state = take_actions!(game_state => c7s);
        let piece_board = game_state.piece_board();

        assert_eq!(
            piece_board.rabbits,
            0b__00000000__11111001__00000010__00000000__00000100__00000000__11111011__00000000
        );
        assert_eq!(
            piece_board.p1_pieces,
            0b__11111111__11111001__00000010__00000000__00000100__00000000__00000000__00000000
        );
        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_action_move_push_must_push_rabbit() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => b7s);

        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::MustCompletePush(Square::new('b', 7), Piece::Rabbit)
        );
    }

    #[test]
    fn test_action_move_push_must_push_elephant() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => a2n);
        let game_state = take_actions!(game_state => p);
        let game_state = take_actions!(game_state => e7s);
        let game_state = take_actions!(game_state => p);
        let game_state = take_actions!(game_state => e8s);

        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::MustCompletePush(Square::new('e', 8), Piece::Elephant)
        );
    }

    #[test]
    fn test_action_place_initial() {
        let game_state = GameState::initial();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, d, c, r]"
        );
    }

    #[test]
    fn test_action_place_elephant() {
        let game_state = GameState::initial();
        let game_state = take_actions!(game_state => e);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[m, h, d, c, r]"
        );
    }

    #[test]
    fn test_action_place_camel() {
        let game_state = GameState::initial();
        let game_state = take_actions!(game_state => m);
        let game_state = take_actions!(game_state => m);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, h, d, c, r]"
        );
    }

    #[test]
    fn test_action_place_horse() {
        let game_state = GameState::initial();
        let game_state = take_actions!(game_state => h);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, d, c, r]"
        );

        let game_state = take_actions!(game_state => h);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, d, c, r]"
        );
    }

    #[test]
    fn test_action_place_dog() {
        let game_state = GameState::initial();
        let game_state = take_actions!(game_state => d);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, d, c, r]"
        );

        let game_state = take_actions!(game_state => d);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, c, r]"
        );
    }

    #[test]
    fn test_action_place_rabbits() {
        let game_state = GameState::initial();

        let game_state = place_8_rabbits(game_state);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, d, c]"
        );
    }

    #[test]
    fn test_action_place_majors() {
        let game_state = GameState::initial();

        let game_state = place_major_pieces(game_state);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[r]");
    }

    #[test]
    fn test_action_place_p2_initial() {
        let game_state = GameState::initial();

        let game_state = place_major_pieces(game_state);
        let game_state = place_8_rabbits(game_state);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, m, h, d, c, r]"
        );
    }

    #[test]
    fn test_action_place_p2_camel() {
        let game_state = GameState::initial();

        let game_state = place_major_pieces(game_state);
        let game_state = place_8_rabbits(game_state);
        let game_state = take_actions!(game_state => m);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e, h, d, c, r]"
        );
    }

    #[test]
    fn test_action_correct_state_after_4_steps() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => a1n, a2n, a3n, a4n);

        let game_state = take_actions!(game_state => d8s);

        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|   r     r   r   |
7|       r         |
6|     x     x     |
5| R   E r         |
4|                 |
3|     x     x     |
2|                 |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );
    }

    #[test]
    fn test_action_correct_state_after_pull_with_trap() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c5n);

        assert_eq!(
            game_state
                .unwrap_play_phase()
                .push_pull_state()
                .as_possible_pull()
                .unwrap(),
            (Square::new('c', 5), Piece::Elephant)
        );
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5w, a1n, a1e, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|       r         |
4|                 |
3|     x     x     |
2|                 |
1| R               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a1n);
        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a2n, a2e, p]");
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|       r         |
4|                 |
3|     x     x     |
2| R               |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => p);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b8e, e8e, g8e, d5e, b8s, d8s, e8s, g8s, d5s, b8w, d8w, g8w, d5w]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|       r         |
4|                 |
3|     x     x     |
2| R               |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );
    }

    #[test]
    fn test_action_correct_state_after_pull_with_trap_accepted() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2| r               |
             1| D               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c5n);

        assert_eq!(
            game_state
                .unwrap_play_phase()
                .push_pull_state()
                .as_possible_pull()
                .unwrap(),
            (Square::new('c', 5), Piece::Elephant)
        );
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a2n, a2e, d5w, a1e, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|       r         |
4|                 |
3|     x     x     |
2| r               |
1| D               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => d5w);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a2n, a2e, a1e, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2| r               |
1| D               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a2e);
        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a1n]");
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2|   r             |
1| D               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a1n);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b8e, e8e, g8e, c5e, b8s, d8s, e8s, g8s, c5s, b8w, d8w, g8w, c5w]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2| D r             |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );
    }

    #[test]
    fn test_action_cant_push_on_last_step() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2| r         D     |
             1| D               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => f2n);
        let game_state = take_actions!(game_state => c5n);

        assert_eq!(
            game_state
                .unwrap_play_phase()
                .push_pull_state()
                .as_possible_pull()
                .unwrap(),
            (Square::new('c', 5), Piece::Elephant)
        );
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a2n, a2e, d5w, a1e, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|       r         |
4|                 |
3|     x     x     |
2| r               |
1| D               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => d5w);
        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a1e, p]");
        assert_eq!(
            game_state.to_string(),
            "1g
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2| r               |
1| D               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a1e);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b8e, e8e, g8e, c5e, a2e, b8s, d8s, e8s, g8s, c5s, a2s, b8w, d8w, g8w, c5w]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2| r               |
1|   D             |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a2s);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b8e, e8e, g8e, c5e, b8s, d8s, e8s, g8s, c5s, b8w, d8w, g8w, c5w, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|   r   r r   r   |
7|                 |
6|     x     x     |
5|     r           |
4|                 |
3|     x     x     |
2|                 |
1| r D             |
 +-----------------+
   a b c d e f g h
"
        );

        assert_eq!(game_state.is_terminal(), None);

        let game_state = take_actions!(game_state => p);
        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_action_cant_push_and_pull_simultaneously() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|                 |
             4| R               |
             3| c   x     x     |
             2| R               |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a4n, a4e, a2e, a2s, a3e]"
        );

        let game_state = take_actions!(game_state => a2e);
        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a3s]");
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|                 |
7|                 |
6|     x     x     |
5|                 |
4| R               |
3| c   x     x     |
2|   R             |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a3s);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b2n, b2e, b2s, a2n, a2s, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|                 |
7|                 |
6|     x     x     |
5|                 |
4| R               |
3|     x     x     |
2| c R             |
1|                 |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => a2s);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[b2w, a1n, a1e, p]"
        );
        assert_eq!(
            game_state.to_string(),
            "1s
 +-----------------+
8|                 |
7|                 |
6|     x     x     |
5|                 |
4| R               |
3|     x     x     |
2|   R             |
1| c               |
 +-----------------+
   a b c d e f g h
"
        );

        let game_state = take_actions!(game_state => p);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a4n, b2n, a4e, b2e, b2w]"
        );
        assert_eq!(
            game_state.to_string(),
            "2g
 +-----------------+
8|                 |
7|                 |
6|     x     x     |
5|                 |
4| R               |
3|     x     x     |
2|   R             |
1| c               |
 +-----------------+
   a b c d e f g h
"
        );
    }

    #[test]
    fn test_can_pass_first_move() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert!(!game_state.can_pass(true));
    }

    #[test]
    fn test_can_pass_during_possible_pull() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c5s);
        assert!(game_state.can_pass(true));
    }

    #[test]
    fn test_can_pass_during_must_push() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => d5s);
        assert!(!game_state.can_pass(true));
    }

    #[test]
    fn test_can_pass_during_place_phase() {
        let game_state = GameState::initial();

        assert!(!game_state.can_pass(true));

        let game_state = take_actions!(game_state => e);
        assert!(!game_state.can_pass(true));
    }

    #[test]
    fn test_can_pass_false_if_same_as_start_of_move_state() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|     E r         |
             4|                 |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert!(!game_state.can_pass(true));

        let game_state = take_actions!(game_state => c5s);
        assert!(game_state.can_pass(true));

        let game_state = take_actions!(game_state => c4n);
        assert!(!game_state.can_pass(true));

        let game_state = take_actions!(game_state => c5s);
        assert!(game_state.can_pass(true));
    }

    #[test]
    fn test_is_terminal_no_winner() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| M r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| m R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), None);
    }

    #[test]
    fn test_is_terminal_mid_turn() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7| R               |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| m               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => a7n);
        assert_eq!(game_state.is_terminal(), None);

        let game_state = take_actions!(game_state => a8s);
        assert_eq!(game_state.is_terminal(), None);

        let game_state = take_actions!(game_state => p);
        assert_eq!(game_state.is_terminal(), None);
    }

    #[test]
    fn test_is_terminal_p1_winner_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| R r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1|   R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p2_winner_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| r R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_is_terminal_p1_winner_as_p2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| R r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1|   R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p2_winner_as_p2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| r R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_is_terminal_p1_and_p2_met_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| R r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| r R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p1_and_p2_met_as_p2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| R r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|                 |
             1| r R             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_is_terminal_p2_lost_rabbits_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E R           |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p1_lost_rabbits_as_p2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|   r             |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E             |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_is_terminal_p1_lost_rabbits_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|   r             |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E             |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_is_terminal_p2_lost_rabbits_as_p2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E R           |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p1_and_p2_lost_rabbits_as_p1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E             |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
    }

    #[test]
    fn test_is_terminal_p1_and_p2_lost_rabbits_as_p2() {
        let game_state: GameState = "
            1g
              +-----------------+
             8|                 |
             7|   e             |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2|   E             |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));
    }

    #[test]
    fn test_action_cant_push_while_frozen() {
        let game_state: GameState = "
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       M r       |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a1n, a1e]");
    }

    #[test]
    fn test_action_cant_push_while_frozen_2() {
        let game_state: GameState = "
            35s
             +-----------------+
            8|     d           |
            7| r e r r h c r r |
            6| H R m r r X c R |
            5| d R E R R h R r |
            4| R         M   C |
            3|     X     X     |
            2|   D       D   H |
            1|             C   |
             +-----------------+
               a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state = take_actions!(game_state => c8w);
        let game_state = take_actions!(game_state => g5s);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[g6s]");
    }

    #[test]
    fn test_action_move_possible_pull_with_valid_pull() {
        let game_state: GameState = "
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|     E m         |
             3|     x     x     |
             2|                 |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c4n);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::PossiblePull(Square::new('c', 4), Piece::Elephant)
        );

        let game_state = take_actions!(game_state => d4w);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::None
        );
    }

    #[test]
    fn test_action_move_pull_into_trap() {
        let game_state: GameState = "
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|     E m         |
             3|     x     x     |
             2|                 |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c4s);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::PossiblePull(Square::new('c', 4), Piece::Elephant)
        );

        let game_state = take_actions!(game_state => d4w);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::None
        );
    }

    #[test]
    fn test_action_move_possible_pull_with_invalid_pull() {
        let game_state: GameState = "
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|     E e         |
             3|     x     x     |
             2|                 |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c4n);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::PossiblePull(Square::new('c', 4), Piece::Elephant)
        );

        let game_state = take_actions!(game_state => d4w);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::MustCompletePush(Square::new('d', 4), Piece::Elephant)
        );
    }

    #[test]
    fn test_action_move_possible_pull_with_invalid_pull_2() {
        let game_state: GameState = "
              +-----------------+
             8|   r   r r   r   |
             7|                 |
             6|     x     x     |
             5|                 |
             4|     M e         |
             3|     x     x     |
             2|                 |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => c4n);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::PossiblePull(Square::new('c', 4), Piece::Camel)
        );

        let game_state = take_actions!(game_state => d4w);
        assert_eq!(
            game_state.unwrap_play_phase().push_pull_state(),
            PushPullState::MustCompletePush(Square::new('d', 4), Piece::Elephant)
        );
    }

    #[test]
    fn test_action_pass() {
        let game_state = initial_play_state();
        let game_state = take_actions!(game_state => d7s);

        assert_eq!(game_state.unwrap_play_phase().step(), 1);
        assert!(game_state.is_p1_turn_to_move());

        let game_state = take_actions!(game_state => p);

        assert_eq!(game_state.unwrap_play_phase().step(), 0);
        assert!(!game_state.is_p1_turn_to_move());
    }

    #[test]
    fn test_valid_actions() {
        let game_state = initial_play_state();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a2n, b2n, c2n, d2n, e2n, f2n, g2n, h2n]"
        );
    }

    #[test]
    fn test_valid_actions_p2() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R R R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[a7s, b7s, c7s, d7s, e7s, f7s, g7s, h7s]"
        );
    }

    #[test]
    fn test_valid_actions_2() {
        let game_state: GameState = "
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x   R x     |
             2| R R R R   R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e3n, a2n, b2n, c2n, d2n, f2n, g2n, h2n, e1n, e3e, d2e, e3w, f2w]"
        );
    }

    #[test]
    fn test_valid_actions_3() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| h c d m e d c h |
             7| r r r r   r r   |
             6|     x   r x   r |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R R R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d7e, g7e, e6e, e8s, h8s, a7s, b7s, c7s, d7s, f7s, g7s, e6s, h6s, f7w, e6w, h6w]"
        );
    }

    #[test]
    fn test_valid_actions_4() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x   E x     |
             2| R R R R   R R R |
             1| H C D M R D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e3n, a2n, b2n, c2n, d2n, f2n, g2n, h2n, e1n, e3e, d2e, e3s, e3w, f2w]"
        );
    }

    #[test]
    fn test_valid_actions_5() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| h c d m r d c h |
             7| r r r r   r r r |
             6|     x   e x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R E R R R |
             1| H C D M R D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e6n, d7e, e6e, e8s, a7s, b7s, c7s, d7s, f7s, g7s, h7s, e6s, f7w, e6w]"
        );
    }

    #[test]
    fn test_valid_actions_6() {
        let game_state: GameState = "
              +-----------------+
             8|   r   r r   r   |
             7| m   h     e   c |
             6|   r x r r x r   |
             5| h   d     c   d |
             4| E   H         M |
             3|   R x R R H R   |
             2| D   C     C   D |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a5n, c5n, h5n, a5e, c5e, c5w, h5w, b3n, d3n, e3n, f3n, g3n, a2n, c2n, h2n, b1n, d1n, e1n, g1n, a4e, c4e, b3e, g3e, a2e, c2e, f2e, b1e, e1e, g1e, a4s, c4s, h4s, a2s, c2s, f2s, h2s, c4w, h4w, b3w, d3w, c2w, f2w, h2w, b1w, d1w, g1w]");
    }

    #[test]
    fn test_valid_actions_7() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|   r   r r   r   |
             7| m   h     e   c |
             6|   r x r r x r   |
             5| h   d     c   d |
             4| E   H         M |
             3|   R x R R H R   |
             2| D   C     C   D |
             1|   R   R R   R   |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a7n, c7n, f7n, h7n, f5n, b8e, e8e, g8e, a7e, c7e, f7e, b6e, e6e, g6e, f5e, b8s, d8s, e8s, g8s, a7s, c7s, f7s, h7s, b6s, d6s, e6s, g6s, f5s, b8w, d8w, g8w, c7w, f7w, h7w, b6w, d6w, g6w, f5w]");
    }

    #[test]
    fn test_valid_actions_frozen_piece_p1() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a1n, a1e]");
    }

    #[test]
    fn test_valid_actions_frozen_piece_p1_2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       m         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d4e, d4s, d4w]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_piece_p1_mid_move() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|     M           |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[c4n, a1n, c4e, a1e, c4s, c4w]"
        );

        let game_state = take_actions!(game_state => c4e);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a1n, a1e, p]");
    }

    #[test]
    fn test_valid_actions_frozen_piece_p2() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       m         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_piece_p2_2() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| r               |
             7|                 |
             6|     x     x     |
             5|       h         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a8e, a8s]");
    }

    #[test]
    fn test_valid_actions_frozen_piece_p2_3() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| r               |
             7|                 |
             6|     x     x     |
             5|       r         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[a8e, a8s]");
    }

    #[test]
    fn test_valid_actions_frozen_piece_p2_4() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       E         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_push_p2_1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d4e, d4s, d4w, d5n, d5e, d5w]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_push_p2_2() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| r               |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       M         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d4e, d4s, d4w, d5n, a8e, d5e, a8s, d5w]"
        );

        let game_state = take_actions!(game_state => d4e);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[d5s]");

        let game_state = take_actions!(game_state => d5s);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e4n, e4e, e4s, d4n, a8e, a8s, d4s, d4w, p]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_push_p2_3() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| r               |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|       R         |
             3|     x c   x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d4e, d4w, d5n, a8e, d5e, d3e, a8s, d3s, d5w, d3w]"
        );

        let game_state = take_actions!(game_state => d4e);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[d3n, d5s]");

        let game_state = take_actions!(game_state => d5s);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e4n, e4e, e4s, d4n, a8e, d3e, a8s, d3s, d4w, d3w, p]"
        );
    }

    #[test]
    fn test_valid_actions_frozen_push_p2_4() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| r               |
             7|                 |
             6|     x     x     |
             5|       e         |
             4|     E R         |
             3|     x c   x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d4e, d5n, a8e, d5e, d3e, a8s, d3s, d5w, d3w]"
        );

        let game_state = take_actions!(game_state => d4e);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[d3n, d5s]");

        let game_state = take_actions!(game_state => d5s);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e4n, e4e, e4s, d4n, a8e, d3e, a8s, d3s, d3w, p]"
        );
    }

    #[test]
    fn test_valid_actions_supported_piece_p2_1() {
        let game_state: GameState = "
             1s
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|     r m         |
             4|     C E         |
             3|     x     x     |
             2|                 |
             1| R               |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, c5w]"
        );
    }

    #[test]
    fn test_valid_actions_matches_first_step() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       m         |
             4|       E         |
             3|     x     x     |
             2|                 |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => d4e);
        let game_state = take_actions!(game_state => d5s);

        let game_state = take_actions!(game_state => d4n);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[]");
    }

    #[test]
    fn test_valid_actions_matches_first_step_2() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|               r |
             7|             d C |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| E             h |
             1|   e           R |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => a2s);
        let game_state = take_actions!(game_state => a1n);

        let game_state = take_actions!(game_state => a2s);

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[p]");
    }

    #[test]
    fn test_valid_actions_matches_is_terminal_if_only_passing() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|       m         |
             4|       E         |
             3|     x     x     |
             2|                 |
             1|                 |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => d4e);
        let game_state = take_actions!(game_state => d5s);

        let game_state = take_actions!(game_state => d4n);

        assert_eq!(game_state.is_terminal(), Some(Terminal::SilverWin));

        assert_eq!(format!("{:?}", game_state.valid_actions()), "[]");
    }

    #[test]
    fn test_valid_actions_repeated_positions() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|               r |
             7|               d |
             6|     x     x     |
             5|       m         |
             4|                 |
             3|     x E   x     |
             2|                 |
             1|               R |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => d3n);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w, h1n, d4e, d4s, d4w, h1w, p]"
        );
        let game_state = take_actions!(game_state => p);

        // First occurance of position

        let game_state = take_actions!(game_state => h7s);
        let game_state = take_actions!(game_state => p);

        let game_state = take_actions!(game_state => d4s);
        let game_state = take_actions!(game_state => p);

        let game_state = take_actions!(game_state => h6n);
        let game_state = take_actions!(game_state => p);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d3n, h1n, d3e, d3s, d3w, h1w]"
        );

        let game_state = take_actions!(game_state => d3n);
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w, h1n, d4e, d4s, d4w, h1w, p]"
        );
        let game_state = take_actions!(game_state => p);

        // Second occurance of position

        let game_state = take_actions!(game_state => h7s);
        let game_state = take_actions!(game_state => p);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w, h1n, d4e, d4s, d4w, h1w]"
        );

        let game_state = take_actions!(game_state => d4s);
        let game_state = take_actions!(game_state => d3s);
        let game_state = take_actions!(game_state => p);

        let game_state = take_actions!(game_state => h6n);
        let game_state = take_actions!(game_state => p);

        // Would be third occurance of position

        let game_state = take_actions!(game_state => d2n);
        let game_state = take_actions!(game_state => d3n);

        // Should not allow pass here
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5n, d5e, d5w, h1n, d4e, d4s, d4w, h1w]"
        );

        let game_state = take_actions!(game_state => d4s);

        // Should not allow last move to be the final repeat
        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[d5s, h1n, d3e, d3w, h1w, p]"
        );
    }

    #[test]
    fn test_valid_actions_does_not_duplicate_piece_that_can_be_both_pushed_and_pull() {
        let game_state: GameState = "
             1g
              +-----------------+
             8|                 |
             7|                 |
             6|     x     x     |
             5|                 |
             4|       E c H     |
             3|     x     x     |
             2|                 |
             1|   e             |
              +-----------------+
                a b c d e f g h"
            .parse()
            .unwrap();

        let game_state = take_actions!(game_state => f4n);

        assert_eq!(
            format!("{:?}", game_state.valid_actions()),
            "[e4n, e4e, e4s, f5n, d4n, f5e, f5s, d4s, f5w, d4w, p]"
        );
    }

    #[test]
    fn test_hash_initial_play_state() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R R R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();

        assert_eq!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_account_for_player_not_equal() {
        let game_state: GameState = "
             1s
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R R R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();

        assert_ne!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_account_for_step_not_equal() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x   R x     |
             2| R R R R   R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();
        let game_state_2 = take_actions!(game_state_2 => e2n);

        assert_ne!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_account_for_step_equal() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x   R x     |
             2| R R R R   R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();
        let game_state_2 = take_actions!(game_state_2 => e2n);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );

        let game_state_2 = take_actions!(game_state_2 => p);
        let game_state_2 = take_actions!(game_state_2 => p);

        assert_eq!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_allow_for_pass_to_equal() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R R R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();

        let game_state_2 = take_actions!(game_state_2 => p);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );

        let game_state_2 = take_actions!(game_state_2 => p);
        assert_eq!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_switch_player_on_pass_for_step() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r r r r r |
             6|     x     x     |
             5|                 |
             4|                 |
             3|     x   R x     |
             2| R R R R   R R R |
             1| H C D M E D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_2 = initial_play_state();
        let game_state_2 = take_actions!(game_state_2 => e2n);

        assert_ne!(
            game_state.transposition_hash(),
            game_state_2.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_account_for_trapped_pieces() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r     r r |
             6|     x E r x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R   R R R |
             1| H C D M   D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_final: GameState = "
             1s
              +-----------------+
             8| h c d m e d c h |
             7| r r r r     r r |
             6|     x   E x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R   R R R |
             1| H C D M   D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state = take_actions!(game_state => e6e);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );

        let game_state = take_actions!(game_state => d6e);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );

        let game_state = take_actions!(game_state => p);
        assert_eq!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );
    }

    #[test]
    fn test_hash_should_account_for_four_actions() {
        let game_state: GameState = "
             1g
              +-----------------+
             8| h c d m e d c h |
             7| r r r r     r r |
             6|     x E r x     |
             5|         r       |
             4|                 |
             3|     x     x     |
             2| R R R R   R R R |
             1| H C D M   D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state_final: GameState = "
             1s
              +-----------------+
             8| h c d m e d c h |
             7| r r r r E   r r |
             6|     x   r x     |
             5|                 |
             4|                 |
             3|     x     x     |
             2| R R R R   R R R |
             1| H C D M   D C H |
              +-----------------+
                a b c d e f g h
            "
        .parse()
        .unwrap();

        let game_state = take_actions!(game_state => e6e);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );

        let game_state = take_actions!(game_state => d6e);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );

        let game_state = take_actions!(game_state => e6n);
        assert_ne!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );

        let game_state = take_actions!(game_state => e5n);
        assert_eq!(
            game_state.transposition_hash(),
            game_state_final.transposition_hash()
        );
    }
}
