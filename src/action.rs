use super::{Direction, Piece, Square};
use anyhow::anyhow;
use std::fmt;
use std::str::FromStr;

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum Action {
    Place(Piece),
    Move(Square, Direction),
    Pass,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self {
            Action::Move(square, direction) => format!("{}{}", square, direction),
            Action::Pass => "p".to_string(),
            Action::Place(piece) => format!("{}", piece),
        };

        write!(f, "{}", action)
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        if chars.len() == 1 {
            if let Some(c) = chars.first() {
                if *c == 'p' {
                    return Ok(Action::Pass);
                } else if let Ok(piece) = c.to_string().parse::<Piece>() {
                    return Ok(Action::Place(piece));
                }
            }
        } else if chars.len() == 3 {
            if let Ok(square) = s[..2].parse::<Square>() {
                if let Ok(dir) = s[2..].parse::<Direction>() {
                    return Ok(Action::Move(square, dir));
                }
            }
        }

        Err(anyhow!("Invalid action"))
    }
}

pub fn map_bit_board_to_squares(board: u64) -> Vec<Square> {
    let mut board = board;
    let mut squares = Vec::with_capacity(board.count_ones() as usize);

    while board != 0 {
        let bit_idx = board.trailing_zeros();
        let square = Square::from_index(bit_idx as u8);
        squares.push(square);

        board ^= 1 << bit_idx;
    }

    squares
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn col_row_to_bit(col: usize, row: usize) -> u64 {
        1 << ((col - 1) + (8 - row) * 8)
    }

    #[test]
    fn test_as_bit_board_a1() {
        let bit = Square::new('a', 1).as_bit_board();
        let col = 1;
        let row = 1;

        assert_eq!(col_row_to_bit(col, row), bit);
    }

    #[test]
    fn test_as_bit_board_a8() {
        let bit = Square::new('a', 8).as_bit_board();
        let col = 1;
        let row = 8;

        assert_eq!(col_row_to_bit(col, row), bit);
    }

    #[test]
    fn test_as_bit_board_h1() {
        let bit = Square::new('h', 1).as_bit_board();
        let col = 8;
        let row = 1;

        assert_eq!(col_row_to_bit(col, row), bit);
    }
    #[test]
    fn test_as_bit_board_h8() {
        let bit = Square::new('h', 8).as_bit_board();
        let col = 8;
        let row = 8;

        assert_eq!(col_row_to_bit(col, row), bit);
    }

    #[test]
    fn test_as_bit_board_e5() {
        let bit = Square::new('e', 5).as_bit_board();
        let col = 5;
        let row = 5;

        assert_eq!(col_row_to_bit(col, row), bit);
    }

    #[test]
    fn test_from_bit_board_a1() {
        let col = 1;
        let row = 1;
        let bit_board = col_row_to_bit(col, row);

        let square = Square::from_bit_board(bit_board);

        assert_eq!(Square::new('a', 1), square);
    }

    #[test]
    fn test_from_bit_board_a8() {
        let col = 1;
        let row = 8;
        let bit_board = col_row_to_bit(col, row);

        let square = Square::from_bit_board(bit_board);

        assert_eq!(Square::new('a', 8), square);
    }

    #[test]
    fn test_from_bit_board_h1() {
        let col = 8;
        let row = 1;
        let bit_board = col_row_to_bit(col, row);

        let square = Square::from_bit_board(bit_board);

        assert_eq!(Square::new('h', 1), square);
    }

    #[test]
    fn test_from_bit_board_h8() {
        let col = 8;
        let row = 8;
        let bit_board = col_row_to_bit(col, row);

        let square = Square::from_bit_board(bit_board);

        assert_eq!(Square::new('h', 8), square);
    }

    #[test]
    fn test_from_bit_board_e5() {
        let col = 5;
        let row = 5;
        let bit_board = col_row_to_bit(col, row);

        let square = Square::from_bit_board(bit_board);

        assert_eq!(Square::new('e', 5), square);
    }

    #[test]
    fn test_to_square_to_bit_board_from_bit_board_back_to_square() {
        let cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let rows = [1, 2, 3, 4, 5, 6, 7, 8];

        for (col, row) in cols.iter().zip(rows.iter()) {
            let orig_square = Square::new(*col, *row);
            let bit_board = orig_square.as_bit_board();
            let square = Square::from_bit_board(bit_board);

            assert_eq!(orig_square, square);
        }
    }

    #[test]
    fn test_piece_precedence_rabbit() {
        assert_eq!(Piece::Rabbit.cmp(&Piece::Cat), Ordering::Less);
        assert_eq!(Piece::Rabbit.cmp(&Piece::Rabbit), Ordering::Equal);
        assert_eq!(Piece::Cat.cmp(&Piece::Rabbit), Ordering::Greater);
    }

    #[test]
    fn test_piece_precedence_cat() {
        assert_eq!(Piece::Cat.cmp(&Piece::Dog), Ordering::Less);
        assert_eq!(Piece::Cat.cmp(&Piece::Cat), Ordering::Equal);
        assert_eq!(Piece::Dog.cmp(&Piece::Cat), Ordering::Greater);
    }

    #[test]
    fn test_piece_precedence_dog() {
        assert_eq!(Piece::Dog.cmp(&Piece::Horse), Ordering::Less);
        assert_eq!(Piece::Dog.cmp(&Piece::Dog), Ordering::Equal);
        assert_eq!(Piece::Horse.cmp(&Piece::Dog), Ordering::Greater);
    }

    #[test]
    fn test_piece_precedence_horse() {
        assert_eq!(Piece::Horse.cmp(&Piece::Camel), Ordering::Less);
        assert_eq!(Piece::Horse.cmp(&Piece::Horse), Ordering::Equal);
        assert_eq!(Piece::Camel.cmp(&Piece::Horse), Ordering::Greater);
    }

    #[test]
    fn test_piece_precedence_camel() {
        assert_eq!(Piece::Camel.cmp(&Piece::Elephant), Ordering::Less);
        assert_eq!(Piece::Camel.cmp(&Piece::Camel), Ordering::Equal);
        assert_eq!(Piece::Elephant.cmp(&Piece::Camel), Ordering::Greater);
    }

    #[test]
    fn test_piece_precedence_elephant() {
        assert_eq!(Piece::Elephant.cmp(&Piece::Elephant), Ordering::Equal);
        assert_eq!(Piece::Elephant.cmp(&Piece::Rabbit), Ordering::Greater);
    }
}
