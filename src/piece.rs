use anyhow::anyhow;
use std::fmt;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Piece {
    Rabbit,
    Cat,
    Dog,
    Horse,
    Camel,
    Elephant,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Rabbit,
        Piece::Cat,
        Piece::Dog,
        Piece::Horse,
        Piece::Camel,
        Piece::Elephant,
    ];
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            Piece::Elephant => 'e',
            Piece::Camel => 'm',
            Piece::Horse => 'h',
            Piece::Dog => 'd',
            Piece::Cat => 'c',
            Piece::Rabbit => 'r',
        };

        write!(f, "{}", piece)
    }
}

impl FromStr for Piece {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        if chars.len() == 1 {
            if let Some(c) = chars.first() {
                let piece = match c {
                    'E' | 'e' => Some(Piece::Elephant),
                    'M' | 'm' => Some(Piece::Camel),
                    'H' | 'h' => Some(Piece::Horse),
                    'D' | 'd' => Some(Piece::Dog),
                    'C' | 'c' => Some(Piece::Cat),
                    'R' | 'r' => Some(Piece::Rabbit),
                    _ => None,
                };

                if let Some(piece) = piece {
                    return Ok(piece);
                }
            }
        }

        Err(anyhow!("Invalid value for piece"))
    }
}
