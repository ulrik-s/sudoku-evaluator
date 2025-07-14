use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};

const FISH_LEN: usize = 3;

pub struct Swordfish;

impl Strategy for Swordfish {
    fn kind(&self) -> StrategyKind {
        StrategyKind::Swordfish
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for digit in board::digits() {
            let rows: Vec<_> = board::row_indices().collect();
            for i in 0..rows.len() {
                for j in i + 1..rows.len() {
                    for k in j + 1..rows.len() {
                        let (r1, r2, r3) = (rows[i], rows[j], rows[k]);
                        let cols1 = board.row_candidate_positions(r1, digit);
                        let cols2 = board.row_candidate_positions(r2, digit);
                        let cols3 = board.row_candidate_positions(r3, digit);
                        let mut union = std::collections::BTreeSet::new();
                        union.extend(cols1.iter());
                        union.extend(cols2.iter());
                        union.extend(cols3.iter());
                        if cols1.len() <= FISH_LEN
                            && cols2.len() <= FISH_LEN
                            && cols3.len() <= FISH_LEN
                            && union.len() == FISH_LEN
                        {
                            let changed = board::row_indices()
                                .filter(|&r| r != r1 && r != r2 && r != r3)
                                .flat_map(|r| union.iter().map(move |&c| (r, c)))
                                .try_fold(false, |acc, (r, c)| {
                                    match board.eliminate_candidate(r, c, digit) {
                                        Some(true) => Ok(true),
                                        Some(false) => Ok(acc),
                                        None => Err(SolverError::Contradiction { row: r, col: c }),
                                    }
                                })?;
                            if changed {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        for digit in board::digits() {
            let cols: Vec<_> = board::col_indices().collect();
            for i in 0..cols.len() {
                for j in i + 1..cols.len() {
                    for k in j + 1..cols.len() {
                        let (c1, c2, c3) = (cols[i], cols[j], cols[k]);
                        let rows1 = board.col_candidate_positions(c1, digit);
                        let rows2 = board.col_candidate_positions(c2, digit);
                        let rows3 = board.col_candidate_positions(c3, digit);
                        let mut union = std::collections::BTreeSet::new();
                        union.extend(rows1.iter());
                        union.extend(rows2.iter());
                        union.extend(rows3.iter());
                        if rows1.len() <= FISH_LEN
                            && rows2.len() <= FISH_LEN
                            && rows3.len() <= FISH_LEN
                            && union.len() == FISH_LEN
                        {
                            let changed = board::col_indices()
                                .filter(|&c| c != c1 && c != c2 && c != c3)
                                .flat_map(|c| union.iter().map(move |&r| (r, c)))
                                .try_fold(false, |acc, (r, c)| {
                                    match board.eliminate_candidate(r, c, digit) {
                                        Some(true) => Ok(true),
                                        Some(false) => Ok(acc),
                                        None => Err(SolverError::Contradiction { row: r, col: c }),
                                    }
                                })?;
                            if changed {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }
}
