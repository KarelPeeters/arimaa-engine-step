/*!
A game engine for the board game [Arimaa](http://arimaa.com/arimaa/). This library
provides the functionality to parse and display board states, generate sets of valid
step based actions as well as take actions on those states.

# Example: Initialize a new game state.

A new game state can be initialized from the GameState

```
use arimaa_engine_step::GameState;

let game_state = GameState::initial();

println!("{}", game_state);
```
# Example: Parsing game states and actions.

A game state can also be read from a string as well as actions.

```
use arimaa_engine_step::{action, board};

let game_state = board!(
    "2g
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
       a b c d e f g h"
);

let action = action!(d2n);
let game_state = game_state.take_action(&action);
```
# Example: Generate valid steps

A game state can be used to generate a valid list of actions. These actions are based on a list of valid steps.

```
use arimaa_engine_step::{action, board};

let game_state = board!("
    2g
     +-----------------+
    8|           r     |
    7|                 |
    6|   R x   e x     |
    5|                 |
    4|                 |
    3|     x     x     |
    2|                 |
    1|                 |
     +-----------------+
       a b c d e f g h");

let valid_actions = game_state.valid_actions();

assert_eq!(&[action!(b6n), action!(b6e), action!(b6w)], &*valid_actions);
```

# Example: Detect terminals

A game state can be used to determine if a terminal state has been reached.

```
use arimaa_engine_step::{Terminal, board, take_actions};

let game_state = board!("
    2g
     +-----------------+
    8|           r     |
    7|                 |
    6|   R x   e x     |
    5|                 |
    4|                 |
    3|     x     x     |
    2|                 |
    1|                 |
     +-----------------+
       a b c d e f g h");

assert_eq!(game_state.is_terminal(), None);

let game_state = take_actions!(game_state => b6n, b7n, p);

assert_eq!(game_state.is_terminal(), Some(Terminal::GoldWin));
```

# Example: Generate moves

Generate a full set of unique moves.

```
use arimaa_engine_step::{take_actions, Action, GameState};
use std::collections::HashSet;

// Start a new game with a basic setup.
let game_state = GameState::initial();
let game_state = take_actions!(game_state => r, r, r, r, r, r, r, r);
let game_state = take_actions!(game_state => c, d, h, c, e, h, d, c);

let game_state = take_actions!(game_state => c, d, h, c, e, h, d, c);
let game_state = take_actions!(game_state => r, r, r, r, r, r, r, r);

fn recurse_actions_for_state(
    game_state: &GameState,
    actions: &[Action],
    moves: &mut Vec<(Vec<Action>, GameState)>,
    hashes: &mut HashSet<u64>,
) {
    let curr_player = game_state.is_p1_turn_to_move();

    // Go through each valid action for the current step in the move.
    for action in game_state.valid_actions() {
        let new_game_state = game_state.take_action(&action);

        // If the state was already explored in another transposition, then skip over that state.
        let game_state_hash = new_game_state.transposition_hash();
        if hashes.contains(&game_state_hash) {
            continue;
        } else {
            hashes.insert(game_state_hash);
        }

        let mut actions = actions.to_vec();
        actions.push(action);

        // If the state is terminal or it is the end of the turn for the player, then it is a full move.
        if matches!(new_game_state.is_terminal(), Some(_)) || curr_player != new_game_state.is_p1_turn_to_move() {
            moves.push((actions, new_game_state));
            continue;
        }

        // A full move has not been reached, it is not terminal and it is still the same players turn.
        // Recurse to check the next set of valid steps in the move.
        recurse_actions_for_state(&new_game_state, &actions, moves, hashes);
    }
}

let mut moves = vec![];
let mut hashes = HashSet::new();
recurse_actions_for_state(&game_state, &[], &mut moves, &mut hashes);

println!("Found {} unique moves", moves.len());
assert_eq!(moves.len(), 2467, "The setup should have 2467 unique moves.");

for (i, (actions, _game_state)) in moves.into_iter().enumerate() {
    println!("{}: {:?}", i, actions);
}
```

# Example: Create a plane out of piece board bits

Creates a slice for a 8x8 board that would be used as an input plane for a neural network by taking
the bits from the piece board plane and setting those matching bits into an array.

```
use arimaa_engine_step::{board, Piece};

let game_state = board!("
    2g
     +-----------------+
    8|           r     |
    7|                 |
    6|   R x   e x     |
    5|                 |
    4|                 |
    3|     x R r x     |
    2|                 |
    1|                 |
     +-----------------+
       a b c d e f g h");

let mut gold_rabbit_bits = game_state.piece_board().bits_for_piece(Piece::Rabbit, true);
let mut plane = [0; 64];

while gold_rabbit_bits != 0 {
    let bit_idx = gold_rabbit_bits.trailing_zeros() as usize;
    let removed_bit_idx = bit_idx;

    plane[removed_bit_idx] = 1;

    gold_rabbit_bits ^= 1 << bit_idx;
}

assert_eq!(
    plane,
    [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]
);
```
*/

#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::unusual_byte_groupings)]

#[macro_use]
mod bit_manip;
mod bit_mask;
mod engine_tests;
mod zobrist_values;

pub mod action;
pub mod constants;
pub mod direction;
pub mod display;
pub mod engine;
pub mod linked_list;
pub mod macros;
pub mod piece;
pub mod square;
pub mod terminal;
pub mod zobrist;

pub use action::*;
pub use constants::*;
pub use direction::*;
pub use display::*;
pub use engine::*;
pub use linked_list::*;
pub use macros::*;
pub use piece::*;
pub use square::*;
pub use terminal::*;
pub use zobrist::Zobrist;
