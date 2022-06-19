use super::bit_manip::single_bit_index;
use super::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use anyhow::anyhow;
use std::fmt;
use std::str::FromStr;

const ASCII_LETTER_A: u8 = 97;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Square(u8);

impl Square {
    pub fn new(column: char, row: usize) -> Self {
        let index = (column as u8 - ASCII_LETTER_A) + (BOARD_HEIGHT - row) as u8 * 8;
        Square(index)
    }

    pub fn from_index(index: u8) -> Self {
        Square(index)
    }

    pub fn from_bit_board(board: u64) -> Self {
        Square(single_bit_index(board as u128) as u8)
    }

    pub fn as_bit_board(&self) -> u64 {
        1 << self.0
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn column_char(&self) -> char {
        let index = self.0 as usize;
        let column = index % BOARD_WIDTH;

        (ASCII_LETTER_A + column as u8) as char
    }

    pub fn row(&self) -> u8 {
        let index = self.0 as usize;
        let row = BOARD_HEIGHT - (index / BOARD_WIDTH) as usize;
        row as u8
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let column = self.column_char();
        let row = self.row();

        write!(f, "{column}{row}", column = column, row = row)
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Square {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        if chars.len() == 2 {
            let column = chars[0];
            let row = chars[1];

            if let Ok(row) = row.to_string().parse() {
                let column_as_num = column as u8 - ASCII_LETTER_A + 1;
                if column_as_num >= 1
                    && column_as_num <= BOARD_WIDTH as u8
                    && (1..=BOARD_HEIGHT).contains(&row)
                {
                    return Ok(Square::new(column, row));
                }
            }
        }

        Err(anyhow!("Invalid value for square"))
    }
}
