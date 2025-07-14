//! Candidate tracking structures used by the Sudoku board.

use super::Digit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CandidateSet(pub(crate) u16);

impl CandidateSet {
    pub fn full() -> Self { Self(0x1FF) }
    pub fn empty() -> Self { Self(0) }
    pub fn contains(self, d: Digit) -> bool { self.0 & (1 << (d - 1)) != 0 }
    pub fn remove(&mut self, d: Digit) { self.0 &= !(1 << (d - 1)); }
    pub fn insert(&mut self, d: Digit) { self.0 |= 1 << (d - 1); }
    pub fn union(self, other: CandidateSet) -> CandidateSet { CandidateSet(self.0 | other.0) }
    pub fn is_empty(self) -> bool { self.0 == 0 }
    pub fn len(self) -> usize { self.into_iter().count() }
    pub fn iter(self) -> CandidateIter { CandidateIter { mask: self.0, digit: 1 } }
}

pub struct CandidateIter {
    mask: u16,
    digit: Digit,
}

impl Iterator for CandidateIter {
    type Item = Digit;
    fn next(&mut self) -> Option<Self::Item> {
        while self.digit <= 9 {
            let d = self.digit;
            self.digit += 1;
            if self.mask & (1 << (d - 1)) != 0 {
                return Some(d);
            }
        }
        None
    }
}

impl IntoIterator for CandidateSet {
    type Item = Digit;
    type IntoIter = CandidateIter;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a> IntoIterator for &'a CandidateSet {
    type Item = Digit;
    type IntoIter = CandidateIter;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl PartialEq<Vec<Digit>> for CandidateSet {
    fn eq(&self, other: &Vec<Digit>) -> bool {
        self.len() == other.len() && other.iter().all(|&d| self.contains(d))
    }
}

impl PartialEq<CandidateSet> for Vec<Digit> {
    fn eq(&self, other: &CandidateSet) -> bool { other == self }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CandidatePositions {
    positions: [usize; 9],
    len: usize,
}

impl CandidatePositions {
    pub fn new() -> Self { Self { positions: [0; 9], len: 0 } }
    pub fn push(&mut self, idx: usize) { self.positions[self.len] = idx; self.len += 1; }
    pub fn len(&self) -> usize { self.len }
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ { self.positions[..self.len].iter().copied() }
}

impl<'a> IntoIterator for &'a CandidatePositions {
    type Item = usize;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, usize>>;
    fn into_iter(self) -> Self::IntoIter { self.positions[..self.len].iter().copied() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CandidateCoords {
    coords: [(usize, usize); 9],
    len: usize,
}

impl CandidateCoords {
    pub fn new() -> Self {
        Self { coords: [(0, 0); 9], len: 0 }
    }
    pub fn push(&mut self, coord: (usize, usize)) {
        self.coords[self.len] = coord;
        self.len += 1;
    }
    pub fn len(&self) -> usize { self.len }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.coords[..self.len].iter().copied()
    }
}

impl<'a> IntoIterator for &'a CandidateCoords {
    type Item = (usize, usize);
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, (usize, usize)>>;
    fn into_iter(self) -> Self::IntoIter {
        self.coords[..self.len].iter().copied()
    }
}

