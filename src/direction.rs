use anyhow::anyhow;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match self {
            Direction::Up => 'n',
            Direction::Right => 'e',
            Direction::Down => 's',
            Direction::Left => 'w',
        };

        write!(f, "{}", dir)
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        if chars.len() == 1 {
            if let Some(c) = chars.first() {
                let direction = match c {
                    'n' => Some(Direction::Up),
                    'e' => Some(Direction::Right),
                    's' => Some(Direction::Down),
                    'w' => Some(Direction::Left),
                    _ => None,
                };

                if let Some(direction) = direction {
                    return Ok(direction);
                }
            }
        }

        Err(anyhow!("Invalid value for direction"))
    }
}
