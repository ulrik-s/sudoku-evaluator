//! Board representation and parsing utilities.
//!
//! The [`Board`] type stores cell values and candidate information. Parsing
//! errors are reported via [`BoardError`].

pub type Digit = u8;

/// Number of rows and columns in a standard Sudoku puzzle.
pub const BOARD_SIZE: usize = 9;
/// Width and height of a single box.
pub const BOX_SIZE: usize = 3;
/// Array containing all valid Sudoku digits.
pub const DIGITS: [Digit; BOARD_SIZE] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

/// Iterator over row indices (0..9).
pub fn row_indices() -> impl Iterator<Item = usize> {
    0..BOARD_SIZE
}

/// Iterator over column indices (0..9).
pub fn col_indices() -> impl Iterator<Item = usize> {
    0..BOARD_SIZE
}

/// Iterator over all Sudoku digits (1..=9).
pub fn digits() -> impl Iterator<Item = Digit> {
    DIGITS.iter().copied()
}

/// Iterator over all unordered pairs of row indices.
pub fn row_pairs() -> impl Iterator<Item = (usize, usize)> {
    row_indices().flat_map(|r1| row_indices().skip(r1 + 1).map(move |r2| (r1, r2)))
}

/// Iterator over all unordered pairs of column indices.
pub fn col_pairs() -> impl Iterator<Item = (usize, usize)> {
    col_indices().flat_map(|c1| col_indices().skip(c1 + 1).map(move |c2| (c1, c2)))
}

mod candidate;
mod board;
mod parser;
mod unit;

pub use candidate::*;
pub use board::Board;
pub use parser::BoardError;
pub use unit::{Unit, UnitIter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display() {
        let puzzle = format!("1{}", ".".repeat(80));
        let board = Board::from_str(&puzzle).unwrap();
        assert_eq!(format!("{}", board), puzzle);
    }

    #[test]
    fn candidates_basic() {
        let puzzle = format!("1{}", ".".repeat(80));
        let mut board = Board::from_str(&puzzle).unwrap();
        let cands = board.candidates(0,1);
        assert_eq!(cands, vec![2,3,4,5,6,7,8,9]);
        board.set(0,1,2);
        assert!(board.candidates(0,1).is_empty());
    }
}
