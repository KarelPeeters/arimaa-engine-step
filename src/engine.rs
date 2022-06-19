use super::bit_manip::first_set_bit;
use super::bit_mask::*;
use super::{map_bit_board_to_squares, Action, Direction, Piece, Square};
use super::{List, Terminal, Zobrist};
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PushPullState {
    ///  Either:
    ///  - The turn started
    ///  - The last move was the completion of a pull.
    ///  - The last move was the completion of a push.
    None,
    /// Currently player's piece was moved last action, next action can possibly be a pull.
    PossiblePull(Square, Piece),
    /// Opponents piece was pushed in the previous step. Follow up step must occupy the empty square.
    MustCompletePush(Square, Piece),
}

#[derive(Clone, Debug)]
pub struct PlayPhase {
    previous_piece_boards_this_move: Vec<PieceBoard>,
    push_pull_state: PushPullState,
    initial_hash_of_move: Zobrist,
    hash_history: List<Zobrist>,
    piece_trapped_this_turn: bool,
}

#[derive(Clone, Debug)]
pub enum Phase {
    PlacePhase,
    PlayPhase(PlayPhase),
}

/// A set of bitboards representing the game state.
/// Bits are in the perspective of player 1.
/// Bits are left to right, top to bottom:
/// - First bit is the top left square (A8).
/// - Second bit is to the right. (B8).
/// - Last bit being the bottom right (H1).
#[derive(Clone, Debug)]
pub struct PieceBoardState {
    pub p1_pieces: u64,
    pub all_pieces: u64,
    pub elephants: u64,
    pub camels: u64,
    pub horses: u64,
    pub dogs: u64,
    pub cats: u64,
    pub rabbits: u64,
}

impl PieceBoardState {
    /// Returns the bits for a specific piece type for a specific player.
    pub fn bits_for_piece(&self, piece: Piece, p1_pieces: bool) -> u64 {
        let player_piece_mask = if p1_pieces {
            self.p1_pieces
        } else {
            !self.p1_pieces & self.all_pieces
        };
        self.bits_by_piece_type(piece) & player_piece_mask
    }

    /// Returns a bit board where each bit represents a current player's piece on that square
    pub fn player_piece_mask(&self, p1_pieces: bool) -> u64 {
        if p1_pieces {
            self.p1_pieces
        } else {
            !self.p1_pieces & self.all_pieces
        }
    }

    // Returns a bit board by piece type. This includes both gold and silver pieces of that type.
    pub fn bits_by_piece_type(&self, piece: Piece) -> u64 {
        match piece {
            Piece::Elephant => self.elephants,
            Piece::Camel => self.camels,
            Piece::Horse => self.horses,
            Piece::Dog => self.dogs,
            Piece::Cat => self.cats,
            Piece::Rabbit => self.rabbits,
        }
    }

    // Returns a bit representing the square where the next piece to be placed will be. This is only relevant during the setup phase.
    pub fn placement_bit(&self) -> u64 {
        let placement_mask = if self.p1_pieces & P1_PLACEMENT_MASK == P1_PLACEMENT_MASK {
            P2_PLACEMENT_MASK
        } else {
            P1_PLACEMENT_MASK
        };
        let squares_to_place = !self.all_pieces & placement_mask;

        first_set_bit(squares_to_place)
    }

    // Returns a bit representing a piece that will be trapped. The piece is on a trap square and is not supported by any friendly pieces.
    // If no pieces will be trapped, then 0.
    pub fn trapped_piece_bits(&self) -> u64 {
        let animal_is_on_trap = animal_is_on_trap(self);

        if animal_is_on_trap {
            let unsupported_piece_bits = both_player_unsupported_piece_bits(self);
            unsupported_piece_bits & TRAP_MASK
        } else {
            0
        }
    }

    // Gets the piece type at the specified square.
    pub fn piece_type_at_square(&self, square: &Square) -> Option<Piece> {
        let square_bit = square.as_bit_board();
        if square_bit & self.all_pieces != 0 {
            Some(piece_type_at_bit(square_bit, self))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct PieceBoard(PieceBoardState);

impl PieceBoard {
    pub fn initial() -> Self {
        Self(PieceBoardState {
            p1_pieces: 0,
            all_pieces: 0,
            elephants: 0,
            camels: 0,
            horses: 0,
            dogs: 0,
            cats: 0,
            rabbits: 0,
        })
    }

    pub fn new(
        p1_pieces: u64,
        elephants: u64,
        camels: u64,
        horses: u64,
        dogs: u64,
        cats: u64,
        rabbits: u64,
    ) -> Self {
        Self(PieceBoardState {
            p1_pieces,
            elephants,
            camels,
            horses,
            dogs,
            cats,
            rabbits,
            all_pieces: elephants | camels | horses | dogs | cats | rabbits,
        })
    }

    pub fn piece_board(&self) -> &PieceBoardState {
        &self.0
    }

    /// Makes the specified move action.
    fn take_action(&self, move_action: &Action) -> (PieceBoardState, bool) {
        let mut piece_board_state = self.0.clone();
        let animal_was_trapped;

        if let Action::Move(square, direction) = move_action {
            Self::move_piece(&mut piece_board_state, square, direction);
            animal_was_trapped = Self::remove_trapped_pieces(&mut piece_board_state);
        } else {
            panic!("Action must be of type Action::Move");
        }

        (piece_board_state, animal_was_trapped)
    }

    /// Shifts a piece from the specified square in the specified direction.
    /// remove_trapped_pieces should be called immediately after each piece move.
    fn move_piece(piece_board_state: &mut PieceBoardState, square: &Square, direction: &Direction) {
        let source_square_bit = square.as_bit_board();
        let pb = piece_board_state;

        pb.elephants = shift_piece_in_direction(pb.elephants, source_square_bit, direction);
        pb.camels = shift_piece_in_direction(pb.camels, source_square_bit, direction);
        pb.horses = shift_piece_in_direction(pb.horses, source_square_bit, direction);
        pb.dogs = shift_piece_in_direction(pb.dogs, source_square_bit, direction);
        pb.cats = shift_piece_in_direction(pb.cats, source_square_bit, direction);
        pb.rabbits = shift_piece_in_direction(pb.rabbits, source_square_bit, direction);
        pb.p1_pieces = shift_piece_in_direction(pb.p1_pieces, source_square_bit, direction);
        pb.all_pieces = shift_piece_in_direction(pb.all_pieces, source_square_bit, direction);
    }

    /// When called, updates the piece board to remove any currently trapped pieces.
    /// This function should be called after every action.
    fn remove_trapped_pieces(piece_board_state: &mut PieceBoardState) -> bool {
        let trapped_animal_bits = piece_board_state.trapped_piece_bits();
        let animal_is_trapped = trapped_animal_bits != 0;

        if animal_is_trapped {
            let untrapped_animal_bits = !trapped_animal_bits;
            piece_board_state.elephants &= untrapped_animal_bits;
            piece_board_state.camels &= untrapped_animal_bits;
            piece_board_state.horses &= untrapped_animal_bits;
            piece_board_state.dogs &= untrapped_animal_bits;
            piece_board_state.cats &= untrapped_animal_bits;
            piece_board_state.rabbits &= untrapped_animal_bits;
            piece_board_state.p1_pieces &= untrapped_animal_bits;
            piece_board_state.all_pieces &= untrapped_animal_bits;
        }

        animal_is_trapped
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    p1_turn_to_move: bool,
    move_number: usize,
    phase: Phase,
    piece_board: PieceBoard,
    hash: Zobrist,
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash.board_state_hash());
        state.finish();
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &GameState) -> bool {
        self.hash.board_state_hash() == other.hash.board_state_hash()
    }
}

impl Eq for GameState {}

impl GameState {
    /// Creates an initial empty board state at the beginning of the game in the setup phase.
    /// It will be gold's turn to place the first piece on an empty board.
    pub fn initial() -> Self {
        GameState {
            p1_turn_to_move: true,
            move_number: 1,
            piece_board: PieceBoard::initial(),
            phase: Phase::PlacePhase,
            hash: Zobrist::initial(),
        }
    }

    /// Creates a new game state from the given information.
    pub fn new(
        p1_turn_to_move: bool,
        move_number: usize,
        phase: Phase,
        piece_board: PieceBoard,
        hash: Zobrist,
    ) -> Self {
        GameState {
            p1_turn_to_move,
            move_number,
            phase,
            piece_board,
            hash,
        }
    }

    /// Takes an action and returns a new game state.
    #[must_use = "This function does not modify the given state. You must use the resultant state."]
    pub fn take_action(&self, action: &Action) -> Self {
        match action {
            Action::Pass => self.pass(),
            Action::Place(piece) => self.place(*piece),
            Action::Move(square, direction) => self.move_piece(square, direction),
        }
    }

    /// Returns Some(Terminal) if the current state is terminal.
    /// Returns None if the current state is not terminal.
    ///
    /// The state is terminal if no steps have been made this turn AND one of the following is true:
    ///     - The last player to move has a rabbit in the goal -> Last player to move wins.
    ///     - The player to move has a rabbit in the goal -> Player to move wins.
    ///     - The player to move has no rabbits -> Last player to move wins.
    ///     - The last player to move has no rabbits -> Player to move wins.
    ///     - The player to move has no valid actions -> Last player to move wins.
    pub fn is_terminal(&self) -> Option<Terminal> {
        self.as_play_phase().and_then(|play_phase| {
            let piece_board = &self.piece_board();

            // The order of checking for win/lose conditions is as follows assuming player A just made the move and player B now needs to move:
            if play_phase.step() > 0 {
                return self.has_move(piece_board);
            }

            // Check if a rabbit of player A reached goal. If so player A wins.
            // Check if a rabbit of player B reached goal. If so player B wins.
            self.rabbit_at_goal(piece_board)
                // Check if player B lost all rabbits. If so player A wins.
                // Check if player A lost all rabbits. If so player B wins.
                .or_else(|| self.lost_all_rabbits(piece_board))
                // Check if player B has no possible move (all pieces are frozen or have no place to move). If so player A wins.
                // Check if the only moves player B has are 3rd time repetitions. If so player A wins.
                .or_else(|| self.has_move(piece_board))
        })
    }

    /// Returns None if the player to move has a valid action.
    /// Returns Some(Terminal) if the player to move has no valid actions.
    ///
    /// This is equivalent to game_state.valid_actions().len() == 0 but is more performant and will short-circuit early.
    #[allow(
        clippy::blocks_in_if_conditions,
        clippy::if_same_then_else,
        clippy::needless_bool
    )]
    pub fn has_move(&self, piece_board: &PieceBoardState) -> Option<Terminal> {
        let has_move = if let Phase::PlayPhase(play_phase) = &self.phase {
            if play_phase.push_pull_state.is_must_complete_push() {
                let valid_actions = self.must_complete_push_actions(piece_board);
                self.has_non_passing_like_action(valid_actions)
            } else if self.can_pass(true) {
                true
            } else if {
                let mut valid_actions = Vec::new();
                self.extend_with_valid_curr_player_piece_moves(&mut valid_actions, piece_board);
                self.has_non_passing_like_action(valid_actions)
            } {
                true
            } else if {
                let mut valid_actions = Vec::new();
                self.extend_with_pull_piece_actions(&mut valid_actions, piece_board);
                self.has_non_passing_like_action(valid_actions)
            } {
                true
            } else if {
                let mut valid_actions = Vec::new();
                self.extend_with_push_piece_actions(&mut valid_actions, piece_board);
                self.has_non_passing_like_action(valid_actions)
            } {
                true
            } else {
                false
            }
        } else {
            true
        };

        if has_move {
            None
        } else if self.p1_turn_to_move {
            Some(Terminal::SilverWin)
        } else {
            Some(Terminal::GoldWin)
        }
    }

    /// Returns a set of valid actions.
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong> Be sure to call game_state.is_terminal() before calling this method. It is not required but is often the case.
    /// </p>
    pub fn valid_actions(&self) -> Vec<Action> {
        self.valid_actions_(true)
    }

    /// Returns a set of valid actions without checking for repititions.
    /// This may be useful when populating a transposition table as no actions will be excluded from 3 fold repititions.
    pub fn valid_actions_no_rep(&self) -> Vec<Action> {
        self.valid_actions_(false)
    }

    /// Returns the piece board for a specified step number.
    /// This may be useful when you are on step n but need the state from an earlier step.
    pub fn piece_board_for_step(&self, step: usize) -> &PieceBoardState {
        if step == self.current_step() {
            return self.piece_board.piece_board();
        }

        let previous_piece_board = &self.unwrap_play_phase().previous_piece_boards_this_move[step];

        previous_piece_board.piece_board()
    }

    /// Returns the piece board for the current step.
    pub fn piece_board(&self) -> &PieceBoardState {
        self.piece_board.piece_board()
    }

    /// True if the current player to move is gold.
    pub fn is_p1_turn_to_move(&self) -> bool {
        self.p1_turn_to_move
    }

    /// Returns the current move number. The Setup phase is always move 1 and the play phase always starts on move 2.
    pub fn move_number(&self) -> usize {
        self.move_number
    }

    /// Returns the current step starting with the first step of a move being 0.
    pub fn current_step(&self) -> usize {
        self.unwrap_play_phase().step()
    }

    /// Returns true if it is the play phase of the game as opposed to the setup phase.
    pub fn is_play_phase(&self) -> bool {
        matches!(&self.phase, Phase::PlayPhase(_))
    }

    /// Checks if the specified action results in a trapped animal.
    /// Returns None if no animal was trapped.
    /// Otherwise returns the square, piece type, and true if the trapped piece is a gold piece.
    pub fn trapped_animal_for_action(&self, action: &Action) -> Option<(Square, Piece, bool)> {
        let mut piece_board_state = self.piece_board().clone();
        if let Action::Move(square, direction) = action {
            PieceBoard::move_piece(&mut piece_board_state, square, direction);
            let trapped_animal_bits = piece_board_state.trapped_piece_bits();
            if trapped_animal_bits != 0 {
                let square = Square::from_bit_board(trapped_animal_bits);
                let piece = piece_board_state.piece_type_at_square(&square).unwrap();
                let is_p1_piece =
                    piece_board_state.bits_for_piece(piece, true) & square.as_bit_board() != 0;

                return Some((square, piece, is_p1_piece));
            }
        }

        None
    }

    /// Returns true if the current player can pass.
    /// The player can only pass if all of the following conditions are true:
    /// - It is not the first step of the move.
    /// - It is not in the middle of a push.
    /// - The current board state does not match the state at the beginning of the move.
    /// - This would not be the third time that this state has occured at the end of a move.
    pub fn can_pass(&self, check_repititions: bool) -> bool {
        self.as_play_phase().map_or(false, |play_phase| {
            play_phase.step() >= 1
                && !play_phase.push_pull_state.is_must_complete_push()
                && (!check_repititions
                    || ((self.unwrap_play_phase().initial_hash_of_move
                        != self.hash.exclude_step(play_phase.step()))
                        && !hash_history_contains_hash_twice(
                            &play_phase.hash_history,
                            &self.hash.pass(play_phase.step()),
                        )))
        })
    }

    /// Returns the Zobrist hash of the current state.
    pub fn transposition_hash(&self) -> u64 {
        match &self.phase {
            Phase::PlayPhase(play_phase) => self
                .hash
                .board_state_hash_with_push_pull_state(play_phase.push_pull_state),
            Phase::PlacePhase => self.hash.board_state_hash(),
        }
    }

    fn valid_actions_(&self, check_repititions: bool) -> Vec<Action> {
        if let Phase::PlayPhase(play_phase) = &self.phase {
            let piece_board = self.piece_board();
            let mut valid_actions = if play_phase.push_pull_state.is_must_complete_push() {
                self.must_complete_push_actions(piece_board)
            } else {
                let mut valid_actions = Vec::with_capacity(50);
                self.extend_with_push_piece_actions(&mut valid_actions, piece_board);
                self.extend_with_pull_piece_actions(&mut valid_actions, piece_board);
                self.extend_with_valid_curr_player_piece_moves(&mut valid_actions, piece_board);

                if self.can_pass(check_repititions) {
                    valid_actions.push(Action::Pass);
                }

                valid_actions
            };

            if check_repititions {
                self.remove_passing_like_actions(&mut valid_actions);
            }

            valid_actions
        } else {
            self.valid_placement()
        }
    }

    fn valid_placement(&self) -> Vec<Action> {
        let mut actions = Vec::with_capacity(6);
        let piece_board = &self.piece_board();
        let curr_player_pieces = self.curr_player_piece_mask(piece_board);

        if piece_board.elephants & curr_player_pieces == 0 {
            actions.push(Action::Place(Piece::Elephant));
        }
        if piece_board.camels & curr_player_pieces == 0 {
            actions.push(Action::Place(Piece::Camel));
        }
        if (piece_board.horses & curr_player_pieces).count_ones() < 2 {
            actions.push(Action::Place(Piece::Horse));
        }
        if (piece_board.dogs & curr_player_pieces).count_ones() < 2 {
            actions.push(Action::Place(Piece::Dog));
        }
        if (piece_board.cats & curr_player_pieces).count_ones() < 2 {
            actions.push(Action::Place(Piece::Cat));
        }
        if (piece_board.rabbits & curr_player_pieces).count_ones() < 8 {
            actions.push(Action::Place(Piece::Rabbit));
        }

        actions
    }

    fn extend_with_valid_curr_player_piece_moves(
        &self,
        valid_actions: &mut Vec<Action>,
        piece_board: &PieceBoardState,
    ) {
        let non_frozen_pieces = self.curr_player_non_frozen_pieces(piece_board);

        for direction in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        {
            let unoccupied_directions = can_move_in_direction(direction, piece_board);
            let invalid_rabbit_moves = self.invalid_rabbit_moves(direction, piece_board);
            let valid_curr_piece_moves =
                unoccupied_directions & non_frozen_pieces & !invalid_rabbit_moves;

            if valid_curr_piece_moves != 0 {
                let squares = map_bit_board_to_squares(valid_curr_piece_moves);
                valid_actions.extend(squares.into_iter().map(|s| Action::Move(s, *direction)));
            }
        }
    }

    fn extend_with_pull_piece_actions(
        &self,
        valid_actions: &mut Vec<Action>,
        piece_board: &PieceBoardState,
    ) {
        if let Some(play_phase) = self.as_play_phase() {
            if let Some((square, piece)) = play_phase.push_pull_state.as_possible_pull() {
                let opp_piece_mask = self.opponent_piece_mask(piece_board);
                let lesser_opp_pieces = self.lesser_pieces(piece, piece_board) & opp_piece_mask;
                let square_bit = square.as_bit_board();

                for direction in [
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ]
                .iter()
                {
                    if shift_pieces_in_direction(lesser_opp_pieces, direction) & square_bit != 0 {
                        let source_opp_piece_square = Square::from_bit_board(
                            shift_pieces_in_opp_direction(square_bit, direction),
                        );
                        let action = Action::Move(source_opp_piece_square, *direction);
                        if !valid_actions.contains(&action) {
                            valid_actions.push(action);
                        }
                    }
                }
            }
        }
    }

    fn extend_with_push_piece_actions(
        &self,
        valid_actions: &mut Vec<Action>,
        piece_board: &PieceBoardState,
    ) {
        if let Some(play_phase) = self.as_play_phase() {
            if play_phase.push_pull_state.can_push() && play_phase.step() < 3 {
                let predator_piece_mask = self.curr_player_non_frozen_pieces(piece_board);
                let opp_piece_mask = self.opponent_piece_mask(piece_board);
                let opp_threatened_pieces =
                    self.threatened_pieces(predator_piece_mask, opp_piece_mask, piece_board);

                if opp_threatened_pieces != 0 {
                    for direction in [
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ]
                    .iter()
                    {
                        let unoccupied_directions = can_move_in_direction(direction, piece_board);
                        let valid_push_moves = unoccupied_directions & opp_threatened_pieces;

                        if valid_push_moves != 0 {
                            let squares = map_bit_board_to_squares(valid_push_moves);
                            valid_actions
                                .extend(squares.into_iter().map(|s| Action::Move(s, *direction)));
                        }
                    }
                }
            }
        }
    }

    fn must_complete_push_actions(&self, piece_board: &PieceBoardState) -> Vec<Action> {
        let play_phase = self.unwrap_play_phase();
        let (square, pushed_piece) = play_phase.push_pull_state.unwrap_must_complete_push();

        let curr_player_non_frozen_piece_mask = self.curr_player_non_frozen_pieces(piece_board);
        let square_bit = square.as_bit_board();

        let mut valid_actions = vec![];
        for direction in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        {
            let pushing_piece_bit = shift_pieces_in_opp_direction(square_bit, direction)
                & curr_player_non_frozen_piece_mask;
            if pushing_piece_bit != 0
                && piece_type_at_bit(pushing_piece_bit, piece_board) > pushed_piece
            {
                valid_actions.push(Action::Move(
                    Square::from_bit_board(pushing_piece_bit),
                    *direction,
                ));
            }
        }

        valid_actions
    }

    fn place(&self, piece: Piece) -> Self {
        let piece_board = &self.piece_board();
        let placement_bit = piece_board.placement_bit();

        let mut new_elephants = piece_board.elephants;
        let mut new_camels = piece_board.camels;
        let mut new_horses = piece_board.horses;
        let mut new_dogs = piece_board.dogs;
        let mut new_cats = piece_board.cats;
        let mut new_rabbits = piece_board.rabbits;

        match piece {
            Piece::Elephant => new_elephants |= placement_bit,
            Piece::Camel => new_camels |= placement_bit,
            Piece::Horse => new_horses |= placement_bit,
            Piece::Dog => new_dogs |= placement_bit,
            Piece::Cat => new_cats |= placement_bit,
            Piece::Rabbit => new_rabbits |= placement_bit,
        }

        let new_p1_pieces = piece_board.p1_pieces
            | if self.p1_turn_to_move {
                placement_bit
            } else {
                0
            };
        let new_piece_board = PieceBoard::new(
            new_p1_pieces,
            new_elephants,
            new_camels,
            new_horses,
            new_dogs,
            new_cats,
            new_rabbits,
        );

        let switch_players = placement_bit == LAST_P1_PLACEMENT_MASK;
        let switch_phases = placement_bit == LAST_P2_PLACEMENT_MASK;
        let new_p1_turn_to_move = if switch_players {
            false
        } else if switch_phases {
            true
        } else {
            self.p1_turn_to_move
        };
        let new_hash = self.hash.place_piece(
            piece,
            Square::from_bit_board(placement_bit),
            self.p1_turn_to_move,
            switch_players,
            switch_phases,
        );
        let new_phase = if switch_phases {
            let hash_history = List::new();
            let hash_history = hash_history.append(new_hash);
            Phase::PlayPhase(PlayPhase::initial(new_hash, hash_history))
        } else {
            Phase::PlacePhase
        };

        let new_move_number = if switch_phases { 2 } else { 1 };

        Self {
            p1_turn_to_move: new_p1_turn_to_move,
            phase: new_phase,
            piece_board: new_piece_board,
            move_number: new_move_number,
            hash: new_hash,
        }
    }

    fn pass(&self) -> Self {
        let hash = self.hash.pass(self.current_step());
        let play_phase = self.unwrap_play_phase();
        let new_hash_history = if play_phase.piece_trapped_this_turn {
            List::new()
        } else {
            play_phase.hash_history.clone()
        };
        let new_hash_history = new_hash_history.append(hash);

        GameState {
            phase: Phase::PlayPhase(PlayPhase::initial(hash, new_hash_history)),
            p1_turn_to_move: !self.p1_turn_to_move,
            move_number: self.move_number + if self.p1_turn_to_move { 0 } else { 1 },
            piece_board: self.piece_board.clone(),
            hash,
        }
    }

    fn move_piece(&self, square: &Square, direction: &Direction) -> Self {
        let curr_play_phase = self.unwrap_play_phase();
        let curr_step = self.current_step();
        let is_last_step = curr_step >= 3;
        let new_action = Action::Move(*square, *direction);
        let (new_piece_board_state, new_animal_was_trapped) =
            self.piece_board.take_action(&new_action);
        let new_p1_turn_to_move = if is_last_step {
            !self.p1_turn_to_move
        } else {
            self.p1_turn_to_move
        };
        let new_step = if is_last_step { 0 } else { curr_step + 1 };
        let new_move_number = self.move_number
            + if is_last_step && new_p1_turn_to_move {
                1
            } else {
                0
            };
        let new_hash =
            self.hash
                .move_piece(self, &new_piece_board_state, new_step, new_p1_turn_to_move);
        let new_hash_history = if new_animal_was_trapped {
            List::new()
        } else {
            curr_play_phase.hash_history.clone()
        };

        let new_play_phase = if is_last_step {
            let hash_history = new_hash_history.append(new_hash);
            PlayPhase::initial(new_hash, hash_history)
        } else {
            let new_previous_piece_boards_this_move = self.next_piece_boards_this_move();
            let new_push_pull_state = self.next_push_pull_state(square, direction);

            PlayPhase {
                initial_hash_of_move: curr_play_phase.initial_hash_of_move,
                push_pull_state: new_push_pull_state,
                hash_history: new_hash_history,
                previous_piece_boards_this_move: new_previous_piece_boards_this_move,
                piece_trapped_this_turn: curr_play_phase.piece_trapped_this_turn
                    | new_animal_was_trapped,
            }
        };

        GameState {
            p1_turn_to_move: new_p1_turn_to_move,
            move_number: new_move_number,
            phase: Phase::PlayPhase(new_play_phase),
            piece_board: PieceBoard(new_piece_board_state),
            hash: new_hash,
        }
    }

    pub fn unwrap_play_phase(&self) -> &PlayPhase {
        self.as_play_phase()
            .expect("Expected phase to be PlayPhase")
    }

    pub fn as_play_phase(&self) -> Option<&PlayPhase> {
        match &self.phase {
            Phase::PlayPhase(play_phase) => Some(play_phase),
            _ => None,
        }
    }

    fn next_piece_boards_this_move(&self) -> Vec<PieceBoard> {
        let play_phase = self.unwrap_play_phase();
        let step = play_phase.step();

        let mut previous_piece_boards_this_move = Vec::with_capacity(step + 1);
        previous_piece_boards_this_move.extend(play_phase.previous_piece_boards_this_move.iter().cloned());
        previous_piece_boards_this_move.push(self.piece_board.clone());
        previous_piece_boards_this_move
    }

    fn next_push_pull_state(&self, square: &Square, direction: &Direction) -> PushPullState {
        let source_square_bit = square.as_bit_board();
        let piece_board = &self.piece_board();
        let is_opponent_piece = self.is_their_piece(source_square_bit, piece_board);
        let play_phase = self.unwrap_play_phase();
        let piece_type_at_bit = piece_type_at_bit(source_square_bit, piece_board);

        // Check if previous move can count as a pull, if so, do that.
        // Otherwise state that it must be followed with a push.
        if is_opponent_piece
            && !self.move_can_be_counted_as_pull(source_square_bit, direction, piece_board)
        {
            PushPullState::MustCompletePush(*square, piece_type_at_bit)
        } else if !is_opponent_piece
            && !play_phase.push_pull_state.is_must_complete_push()
            && piece_type_at_bit != Piece::Rabbit
        {
            PushPullState::PossiblePull(*square, piece_type_at_bit)
        } else {
            PushPullState::None
        }
    }

    fn move_can_be_counted_as_pull(
        &self,
        new_move_square_bit: u64,
        direction: &Direction,
        piece_board: &PieceBoardState,
    ) -> bool {
        let play_phase = self.unwrap_play_phase();
        if let PushPullState::PossiblePull(prev_move_square, my_piece) = &play_phase.push_pull_state
        {
            if prev_move_square.as_bit_board() == shift_in_direction(new_move_square_bit, direction)
            {
                let their_piece = piece_type_at_bit(new_move_square_bit, piece_board);
                if my_piece > &their_piece {
                    return true;
                }
            }
        }

        false
    }

    fn is_their_piece(&self, square_bit: u64, piece_board: &PieceBoardState) -> bool {
        let is_p1_piece = square_bit & piece_board.p1_pieces != 0;
        self.p1_turn_to_move ^ is_p1_piece
    }

    fn curr_player_non_frozen_pieces(&self, piece_board: &PieceBoardState) -> u64 {
        let opp_piece_mask = self.opponent_piece_mask(piece_board);
        let curr_player_piece_mask = !opp_piece_mask & piece_board.all_pieces;
        let threatened_pieces =
            self.threatened_pieces(opp_piece_mask, curr_player_piece_mask, piece_board);

        curr_player_piece_mask & (!threatened_pieces | supported_pieces(curr_player_piece_mask))
    }

    fn threatened_pieces(
        &self,
        predator_piece_mask: u64,
        prey_piece_mask: u64,
        piece_board: &PieceBoardState,
    ) -> u64 {
        let predator_elephant_influence =
            influenced_squares(piece_board.elephants & predator_piece_mask);
        let predator_camel_influence = influenced_squares(piece_board.camels & predator_piece_mask);
        let predator_horse_influence = influenced_squares(piece_board.horses & predator_piece_mask);
        let predator_dog_influence = influenced_squares(piece_board.dogs & predator_piece_mask);
        let predator_cat_influence = influenced_squares(piece_board.cats & predator_piece_mask);

        let camel_threats = predator_elephant_influence;
        let horse_threats = camel_threats | predator_camel_influence;
        let dog_threats = horse_threats | predator_horse_influence;
        let cat_threats = dog_threats | predator_dog_influence;
        let rabbit_threats = cat_threats | predator_cat_influence;

        let threatened_pieces = (piece_board.camels & camel_threats)
            | (piece_board.horses & horse_threats)
            | (piece_board.dogs & dog_threats)
            | (piece_board.cats & cat_threats)
            | (piece_board.rabbits & rabbit_threats);

        threatened_pieces & prey_piece_mask
    }

    fn curr_player_piece_mask(&self, piece_board: &PieceBoardState) -> u64 {
        if self.p1_turn_to_move {
            piece_board.p1_pieces
        } else {
            !piece_board.p1_pieces & piece_board.all_pieces
        }
    }

    fn opponent_piece_mask(&self, piece_board: &PieceBoardState) -> u64 {
        if self.p1_turn_to_move {
            !piece_board.p1_pieces & piece_board.all_pieces
        } else {
            piece_board.p1_pieces
        }
    }

    fn rabbit_at_goal(&self, piece_board: &PieceBoardState) -> Option<Terminal> {
        let p1_objective_met = piece_board.p1_pieces & piece_board.rabbits & P1_OBJECTIVE_MASK != 0;
        let p2_objective_met =
            !piece_board.p1_pieces & piece_board.rabbits & P2_OBJECTIVE_MASK != 0;

        if p1_objective_met || p2_objective_met {
            // Objective is opposite of the player to move since we are checking if there is a winner after the turn is complete.
            // Logic should include the condition of if both players have a rabbit at the goal. In that case the player who was last to move wins.
            let last_to_move_is_p1 = !self.p1_turn_to_move;
            let last_to_move_objective_met = if last_to_move_is_p1 {
                p1_objective_met
            } else {
                p2_objective_met
            };
            let p1_won = !(last_to_move_is_p1 ^ last_to_move_objective_met);
            Some(if p1_won {
                Terminal::GoldWin
            } else {
                Terminal::SilverWin
            })
        } else {
            None
        }
    }

    fn lost_all_rabbits(&self, piece_board: &PieceBoardState) -> Option<Terminal> {
        let p1_lost_rabbits = piece_board.p1_pieces & piece_board.rabbits == 0;
        let p2_lost_rabbits = !piece_board.p1_pieces & piece_board.rabbits == 0;

        // Check if player B lost all rabbits. If so player A wins.
        // Check if player A lost all rabbits. If so player B wins.

        if p1_lost_rabbits || p2_lost_rabbits {
            // Objective is opposite of the player to move since we are checking if there is a winner after the turn is complete.
            // Logic should include the condition of if both players lost their rabbits. In that case the player who was last to move wins.
            let last_to_move_is_p1 = !self.p1_turn_to_move;
            let last_to_move_objective_met = if last_to_move_is_p1 {
                p2_lost_rabbits
            } else {
                p1_lost_rabbits
            };
            let p1_won = !(last_to_move_is_p1 ^ last_to_move_objective_met);
            Some(if p1_won {
                Terminal::GoldWin
            } else {
                Terminal::SilverWin
            })
        } else {
            None
        }
    }

    #[allow(clippy::let_and_return)]
    fn invalid_rabbit_moves(&self, direction: &Direction, piece_board: &PieceBoardState) -> u64 {
        let backward_direction = if self.p1_turn_to_move {
            Direction::Down
        } else {
            Direction::Up
        };

        if *direction == backward_direction {
            let players_rabbits = if self.p1_turn_to_move {
                piece_board.p1_pieces
            } else {
                !piece_board.p1_pieces
            } & piece_board.rabbits;
            players_rabbits
        } else {
            0
        }
    }

    fn lesser_pieces(&self, piece: Piece, piece_board: &PieceBoardState) -> u64 {
        match piece {
            Piece::Rabbit => 0,
            Piece::Cat => piece_board.rabbits,
            Piece::Dog => piece_board.rabbits | piece_board.cats,
            Piece::Horse => piece_board.rabbits | piece_board.cats | piece_board.dogs,
            Piece::Camel => {
                piece_board.rabbits | piece_board.cats | piece_board.dogs | piece_board.horses
            }
            Piece::Elephant => {
                piece_board.rabbits
                    | piece_board.cats
                    | piece_board.dogs
                    | piece_board.horses
                    | piece_board.camels
            }
        }
    }

    fn remove_passing_like_actions(&self, valid_actions: &mut Vec<Action>) {
        let play_phase = self.unwrap_play_phase();
        if play_phase.step() == 3 && !play_phase.piece_trapped_this_turn {
            valid_actions.retain(|action| !self.is_passing_like_action(action));
        }
    }

    fn is_passing_like_action(&self, action: &Action) -> bool {
        let play_phase = self.unwrap_play_phase();
        let initial_hash_of_move = play_phase.initial_hash_of_move;
        let hash_history = &play_phase.hash_history;

        if let Action::Move(_, _) = action {
            let new_piece_board = &self.piece_board.take_action(action).0;
            let new_hash_no_player_switch =
                self.hash
                    .move_piece(self, new_piece_board, 0, self.is_p1_turn_to_move());
            let new_hash_switch_players =
                self.hash
                    .move_piece(self, new_piece_board, 0, !self.is_p1_turn_to_move());

            if new_hash_no_player_switch == initial_hash_of_move
                || hash_history_contains_hash_twice(hash_history, &new_hash_switch_players)
            {
                return true;
            }
        }

        false
    }

    fn has_non_passing_like_action(&self, valid_actions: Vec<Action>) -> bool {
        if valid_actions.is_empty() {
            return false;
        }

        let play_phase = self.unwrap_play_phase();
        if play_phase.step() < 3 || play_phase.piece_trapped_this_turn {
            return true;
        }

        for action in valid_actions.iter() {
            if !self.is_passing_like_action(action) {
                return true;
            }
        }

        false
    }
}

fn piece_type_at_bit(square_bit: u64, piece_board: &PieceBoardState) -> Piece {
    if piece_board.rabbits & square_bit != 0 {
        Piece::Rabbit
    } else if piece_board.elephants & square_bit != 0 {
        Piece::Elephant
    } else if piece_board.camels & square_bit != 0 {
        Piece::Camel
    } else if piece_board.horses & square_bit != 0 {
        Piece::Horse
    } else if piece_board.dogs & square_bit != 0 {
        Piece::Dog
    } else {
        Piece::Cat
    }
}

fn animal_is_on_trap(piece_board: &PieceBoardState) -> bool {
    (piece_board.all_pieces & TRAP_MASK) != 0
}

fn influenced_squares(piece_board: u64) -> u64 {
    shift_pieces_up!(piece_board)
        | shift_pieces_right!(piece_board)
        | shift_pieces_down!(piece_board)
        | shift_pieces_left!(piece_board)
}

fn both_player_unsupported_piece_bits(piece_board: &PieceBoardState) -> u64 {
    piece_board.all_pieces & !both_player_supported_pieces(piece_board)
}

fn both_player_supported_pieces(piece_board: &PieceBoardState) -> u64 {
    let p1_pieces = piece_board.p1_pieces;
    let p2_pieces = piece_board.all_pieces & !p1_pieces;

    supported_pieces(p1_pieces) | supported_pieces(p2_pieces)
}

fn supported_pieces(piece_bits: u64) -> u64 {
    let up_supported_pieces = piece_bits & shift_pieces_up!(piece_bits);
    let right_supported_pieces = piece_bits & shift_pieces_right!(piece_bits);
    let down_supported_pieces = piece_bits & shift_pieces_down!(piece_bits);
    let left_supported_pieces = piece_bits & shift_pieces_left!(piece_bits);

    up_supported_pieces | right_supported_pieces | down_supported_pieces | left_supported_pieces
}

fn can_move_in_direction(direction: &Direction, piece_board: &PieceBoardState) -> u64 {
    let empty_squares = !piece_board.all_pieces;
    shift_pieces_in_opp_direction(empty_squares, direction)
}

fn shift_piece_in_direction(
    piece_board: u64,
    source_square_bit: u64,
    direction: &Direction,
) -> u64 {
    shift_in_direction(piece_board & source_square_bit, direction)
        | piece_board & !source_square_bit
}

fn shift_pieces_in_opp_direction(bits: u64, direction: &Direction) -> u64 {
    match direction {
        Direction::Up => shift_pieces_down!(bits),
        Direction::Right => shift_pieces_left!(bits),
        Direction::Down => shift_pieces_up!(bits),
        Direction::Left => shift_pieces_right!(bits),
    }
}

fn shift_pieces_in_direction(bits: u64, direction: &Direction) -> u64 {
    match direction {
        Direction::Up => shift_pieces_up!(bits),
        Direction::Right => shift_pieces_right!(bits),
        Direction::Down => shift_pieces_down!(bits),
        Direction::Left => shift_pieces_left!(bits),
    }
}

fn shift_in_direction(bits: u64, direction: &Direction) -> u64 {
    match direction {
        Direction::Up => shift_up!(bits),
        Direction::Right => shift_right!(bits),
        Direction::Down => shift_down!(bits),
        Direction::Left => shift_left!(bits),
    }
}

fn hash_history_contains_hash_twice(hash_history: &List<Zobrist>, hash: &Zobrist) -> bool {
    hash_history.iter().filter(|h| *h == hash).count() >= 2
}

impl PushPullState {
    fn is_must_complete_push(&self) -> bool {
        matches!(self, PushPullState::MustCompletePush(_, _))
    }

    fn unwrap_must_complete_push(&self) -> (Square, Piece) {
        match self {
            PushPullState::MustCompletePush(square, piece) => (*square, *piece),
            _ => panic!("Expected PushPullState to be MustCompletePush"),
        }
    }

    pub fn as_possible_pull(&self) -> Option<(Square, Piece)> {
        match self {
            PushPullState::PossiblePull(square, piece) => Some((*square, *piece)),
            _ => None,
        }
    }

    fn can_push(&self) -> bool {
        // We can't push another piece if we are already obligated to push another
        !matches!(self, PushPullState::MustCompletePush(_, _))
    }
}

impl PlayPhase {
    pub fn initial(initial_hash_of_move: Zobrist, hash_history: List<Zobrist>) -> Self {
        PlayPhase {
            previous_piece_boards_this_move: Vec::with_capacity(0),
            push_pull_state: PushPullState::None,
            initial_hash_of_move,
            hash_history,
            piece_trapped_this_turn: false,
        }
    }

    pub fn new(
        initial_hash_of_move: Zobrist,
        hash_history: List<Zobrist>,
        previous_piece_boards_this_move: Vec<PieceBoard>,
        push_pull_state: PushPullState,
        piece_trapped_this_turn: bool,
    ) -> Self {
        PlayPhase {
            previous_piece_boards_this_move,
            push_pull_state,
            initial_hash_of_move,
            hash_history,
            piece_trapped_this_turn,
        }
    }

    pub fn previous_piece_boards(&self) -> &[PieceBoard] {
        &self.previous_piece_boards_this_move
    }

    pub fn push_pull_state(&self) -> PushPullState {
        self.push_pull_state
    }

    pub fn piece_trapped_this_turn(&self) -> bool {
        self.piece_trapped_this_turn
    }

    pub fn step(&self) -> usize {
        self.previous_piece_boards_this_move.len()
    }
}
