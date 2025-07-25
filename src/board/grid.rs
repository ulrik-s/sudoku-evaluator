//! Sudoku board implementation.

use std::fmt;

use super::{Digit, candidate::*, unit::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
    pub value: Option<Digit>,
    removed: u16,
}

impl Cell {
    pub(crate) fn new(value: Option<Digit>) -> Self {
        Cell { value, removed: 0 }
    }
}

/// 9x9 Sudoku board with candidate management helpers.
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    pub(crate) fn new(cells: [[Cell; 9]; 9]) -> Self {
        Self { cells }
    }
    pub fn get(&self, r: usize, c: usize) -> Option<Digit> {
        self.cells[r][c].value
    }

    pub fn set(&mut self, r: usize, c: usize, val: Digit) {
        self.cells[r][c].value = Some(val);
        self.cells[r][c].removed = 0;
    }

    pub fn candidates(&self, r: usize, c: usize) -> CandidateSet {
        if self.get(r, c).is_some() {
            return CandidateSet::empty();
        }
        let mut set = CandidateSet::full();
        for d in self.row_values(r) {
            set.remove(d);
        }
        for d in self.col_values(c) {
            set.remove(d);
        }
        for d in self.box_values(r - r % 3, c - c % 3) {
            set.remove(d);
        }
        let removed = self.cells[r][c].removed;
        for d in 1..=9 {
            if removed & (1 << (d - 1)) != 0 {
                set.remove(d);
            }
        }
        set
    }

    pub fn eliminate_candidate(&mut self, r: usize, c: usize, d: Digit) -> Option<bool> {
        if self.get(r, c).is_some() {
            return Some(false);
        }
        let mask = 1 << (d - 1);
        if self.cells[r][c].removed & mask != 0 {
            return Some(false);
        }
        self.cells[r][c].removed |= mask;
        if self.candidates(r, c).is_empty() {
            None
        } else {
            Some(true)
        }
    }

    pub fn row_candidate_positions(&self, r: usize, digit: Digit) -> CandidatePositions {
        let mut pos = CandidatePositions::new();
        for c in 0..9 {
            if self.get(r, c).is_none() && self.candidates(r, c).contains(digit) {
                pos.push(c);
            }
        }
        pos
    }

    pub fn col_candidate_positions(&self, c: usize, digit: Digit) -> CandidatePositions {
        let mut pos = CandidatePositions::new();
        for r in 0..9 {
            if self.get(r, c).is_none() && self.candidates(r, c).contains(digit) {
                pos.push(r);
            }
        }
        pos
    }

    pub fn candidate_coords(&self, unit: Unit, digit: Digit) -> CandidateCoords {
        let mut coords = CandidateCoords::new();
        for (r, c) in self.unit_iter(unit) {
            if self.get(r, c).is_none() && self.candidates(r, c).contains(digit) {
                coords.push((r, c));
            }
        }
        coords
    }

    pub fn unsolved_in_unit(
        &self,
        unit: Unit,
    ) -> impl Iterator<Item = ((usize, usize), CandidateSet)> + '_ {
        self.unit_iter(unit)
            .filter(|&(r, c)| self.get(r, c).is_none())
            .map(|(r, c)| ((r, c), self.candidates(r, c)))
    }

    pub fn unit_iter(&self, unit: Unit) -> UnitIter {
        let coords = match unit {
            Unit::Row(r) => std::array::from_fn(|i| (r, i)),
            Unit::Col(c) => std::array::from_fn(|i| (i, c)),
            Unit::Box(r, c) => std::array::from_fn(|i| (r + i / 3, c + i % 3)),
        };
        UnitIter::new(coords)
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..9).flat_map(|r| (0..9).map(move |c| (r, c)))
    }

    /// Iterate over every unsolved cell on the board.
    pub fn unsolved_cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.cells().filter(|&(r, c)| self.get(r, c).is_none())
    }

    /// Iterate over every cell on the board in row-major order.
    ///
    /// The provided closure receives the row, column and current value
    /// of the cell. It is similar to calling `map` on the coordinates
    /// iterator but avoids repeating the looping logic in each strategy.
    pub fn for_each_cell<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize, Option<Digit>),
    {
        for r in 0..9 {
            for c in 0..9 {
                f(r, c, self.get(r, c));
            }
        }
    }

    /// Mutable variant of [`for_each_cell`] that allows the closure to
    /// return a `Result`. Returning `Ok(true)` from the closure stops the
    /// iteration early and causes this function to return `Ok(true)`.
    pub fn try_for_each_cell_mut<F, E>(&mut self, mut f: F) -> Result<bool, E>
    where
        F: FnMut(&mut Board, usize, usize) -> Result<bool, E>,
    {
        for r in 0..9 {
            for c in 0..9 {
                if f(self, r, c)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Iterate over every cell on the board and allow the closure to
    /// return a `Result`. Returning `Ok(true)` stops the iteration early.
    pub fn try_for_each_cell<F, E>(&self, mut f: F) -> Result<bool, E>
    where
        F: FnMut(usize, usize, Option<Digit>) -> Result<bool, E>,
    {
        for r in 0..9 {
            for c in 0..9 {
                if f(r, c, self.get(r, c))? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Mutable iteration without early exit for each cell.
    pub fn for_each_cell_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Board, usize, usize),
    {
        for r in 0..9 {
            for c in 0..9 {
                f(self, r, c);
            }
        }
    }

    /// Iterate over a specific unit and run a closure on each cell.
    pub fn for_each_in_unit<F>(&self, unit: Unit, mut f: F)
    where
        F: FnMut(usize, usize, Option<Digit>),
    {
        for (r, c) in self.unit_iter(unit) {
            f(r, c, self.get(r, c));
        }
    }

    /// Mutable variant of [`for_each_in_unit`] that allows early exit.
    pub fn try_for_each_in_unit_mut<F, E>(&mut self, unit: Unit, mut f: F) -> Result<bool, E>
    where
        F: FnMut(&mut Board, usize, usize) -> Result<bool, E>,
    {
        for (r, c) in self.unit_iter(unit) {
            if f(self, r, c)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Iterate over every box and invoke the closure with the box unit.
    pub fn for_each_box<F>(&self, mut f: F)
    where
        F: FnMut(Unit),
    {
        for (r, c) in super::box_indices() {
            f(Unit::Box(r, c));
        }
    }

    /// Iterate over every box and digit combination.
    pub fn for_each_box_digit<F>(&self, mut f: F)
    where
        F: FnMut(Unit, Digit),
    {
        for (r, c) in super::box_indices() {
            for digit in super::digits() {
                f(Unit::Box(r, c), digit);
            }
        }
    }

    /// Mutable variant of [`for_each_box_digit`] with early exit.
    pub fn try_for_each_box_digit_mut<F, E>(&mut self, mut f: F) -> Result<bool, E>
    where
        F: FnMut(&mut Board, Unit, Digit) -> Result<bool, E>,
    {
        for (r, c) in super::box_indices() {
            for digit in super::digits() {
                if f(self, Unit::Box(r, c), digit)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn peer_coords(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        use std::collections::HashSet;
        let br = r / 3 * 3;
        let bc = c / 3 * 3;
        let peers_row = (0..9).filter(move |&cc| cc != c).map(move |cc| (r, cc));
        let peers_col = (0..9).filter(move |&rr| rr != r).map(move |rr| (rr, c));
        let peers_box = (br..br + 3)
            .flat_map(move |rr| (bc..bc + 3).map(move |cc| (rr, cc)))
            .filter(move |&(rr, cc)| rr != r || cc != c);
        peers_row
            .chain(peers_col)
            .chain(peers_box)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub(crate) fn row_values(&self, r: usize) -> impl Iterator<Item = Digit> + '_ {
        self.cells[r].iter().filter_map(|c| c.value)
    }

    pub(crate) fn col_values(&self, c: usize) -> impl Iterator<Item = Digit> + '_ {
        self.cells.iter().filter_map(move |row| row[c].value)
    }

    pub(crate) fn box_values(&self, r: usize, c: usize) -> impl Iterator<Item = Digit> + '_ {
        (r..r + 3).flat_map(move |rr| (c..c + 3).filter_map(move |cc| self.get(rr, cc)))
    }

    pub(crate) fn unique<I: IntoIterator<Item = Digit>>(vals: I) -> bool {
        let mut seen = [false; 10];
        for d in vals {
            if seen[d as usize] {
                return false;
            }
            seen[d as usize] = true;
        }
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..9 {
            for c in 0..9 {
                match self.get(r, c) {
                    Some(d) => write!(f, "{}", d)?,
                    None => write!(f, ".")?,
                }
            }
        }
        Ok(())
    }
}
