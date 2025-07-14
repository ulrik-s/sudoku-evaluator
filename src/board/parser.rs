use super::{board::Board, Digit};
use std::fmt;

/// Errors that can occur while parsing a puzzle string into a [`Board`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardError {
    /// The provided puzzle string did not contain exactly 81 characters.
    InvalidLength(usize),
    /// An unexpected character was encountered at the given index.
    InvalidChar(char, usize),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::InvalidLength(len) => write!(f, "expected 81 chars, got {}", len),
            BoardError::InvalidChar(ch, idx) => write!(f, "invalid char '{}' at position {}", ch, idx),
        }
    }
}

impl std::error::Error for BoardError {}

impl Board {
    pub fn from_str(puzzle: &str) -> Result<Self, BoardError> {
        if puzzle.len() != 81 {
            return Err(BoardError::InvalidLength(puzzle.len()));
        }
        let mut cells = [[super::board::Cell::new(None); 9]; 9];
        puzzle.chars().enumerate().try_for_each(|(idx, ch)| {
            let r = idx / 9;
            let c = idx % 9;
            cells[r][c] = match ch {
                '1'..='9' => super::board::Cell::new(Some(ch.to_digit(10).unwrap() as Digit)),
                '.' | '0' => super::board::Cell::new(None),
                _ => return Err(BoardError::InvalidChar(ch, idx)),
            };
            Ok::<_, BoardError>(())
        })?;
        Ok(Board::new(cells))
    }

    pub fn is_valid(&self) -> bool {
        (0..9).all(|r| Board::unique(self.row_values(r))) &&
        (0..9).all(|c| Board::unique(self.col_values(c))) &&
        (0..3)
            .flat_map(|br| (0..3).map(move |bc| (br*3, bc*3)))
            .all(|(r,c)| Board::unique(self.box_values(r,c)))
    }

    pub fn is_solved(&self) -> bool {
        self.cells().all(|(r,c)| self.get(r,c).is_some()) && self.is_valid()
    }
}

