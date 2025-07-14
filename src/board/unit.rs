//! Abstractions for iterating over rows, columns and boxes.

#[derive(Clone, Copy)]
pub enum Unit {
    Row(usize),
    Col(usize),
    Box(usize, usize),
}

impl Unit {
    pub fn all() -> impl Iterator<Item = Unit> {
        (0..9)
            .map(Unit::Row)
            .chain((0..9).map(Unit::Col))
            .chain((0..3).flat_map(|r| (0..3).map(move |c| Unit::Box(r * 3, c * 3))))
    }

    /// Iterator over all box units.
    pub fn boxes() -> impl Iterator<Item = Unit> {
        super::box_indices().map(|(r, c)| Unit::Box(r, c))
    }

    /// Check whether the given coordinate belongs to this unit.
    pub fn contains(&self, r: usize, c: usize) -> bool {
        match *self {
            Unit::Row(row) => row == r,
            Unit::Col(col) => col == c,
            Unit::Box(br, bc) => {
                r >= br && r < br + super::BOX_SIZE && c >= bc && c < bc + super::BOX_SIZE
            }
        }
    }
}

pub struct UnitIter {
    coords: [(usize, usize); 9],
    idx: usize,
}

impl UnitIter {
    pub(crate) fn new(coords: [(usize, usize); 9]) -> Self {
        UnitIter { coords, idx: 0 }
    }
}

impl Iterator for UnitIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 9 {
            let res = self.coords[self.idx];
            self.idx += 1;
            Some(res)
        } else {
            None
        }
    }
}
