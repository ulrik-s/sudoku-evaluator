use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};

const FISH_LEN: usize = 4;

pub struct Jellyfish;

impl Strategy for Jellyfish {
    fn kind(&self) -> StrategyKind {
        StrategyKind::Jellyfish
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for digit in board::digits() {
            let rows: Vec<_> = board::row_indices().collect();
            for i in 0..rows.len() {
                for j in i + 1..rows.len() {
                    for k in j + 1..rows.len() {
                        for l in k + 1..rows.len() {
                            let (r1, r2, r3, r4) = (rows[i], rows[j], rows[k], rows[l]);
                            let cols1 = board.row_candidate_positions(r1, digit);
                            let cols2 = board.row_candidate_positions(r2, digit);
                            let cols3 = board.row_candidate_positions(r3, digit);
                            let cols4 = board.row_candidate_positions(r4, digit);
                            let mut union = std::collections::BTreeSet::new();
                            union.extend(cols1.iter());
                            union.extend(cols2.iter());
                            union.extend(cols3.iter());
                            union.extend(cols4.iter());
                            if cols1.len() <= FISH_LEN
                                && cols2.len() <= FISH_LEN
                                && cols3.len() <= FISH_LEN
                                && cols4.len() <= FISH_LEN
                                && union.len() == FISH_LEN
                            {
                                let changed = board::row_indices()
                                    .filter(|&r| r != r1 && r != r2 && r != r3 && r != r4)
                                    .flat_map(|r| union.iter().map(move |&c| (r, c)))
                                    .try_fold(false, |acc, (r, c)| {
                                        match board.eliminate_candidate(r, c, digit) {
                                            Some(true) => Ok(true),
                                            Some(false) => Ok(acc),
                                            None => {
                                                Err(SolverError::Contradiction { row: r, col: c })
                                            }
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
        }

        for digit in board::digits() {
            let cols: Vec<_> = board::col_indices().collect();
            for i in 0..cols.len() {
                for j in i + 1..cols.len() {
                    for k in j + 1..cols.len() {
                        for l in k + 1..cols.len() {
                            let (c1, c2, c3, c4) = (cols[i], cols[j], cols[k], cols[l]);
                            let rows1 = board.col_candidate_positions(c1, digit);
                            let rows2 = board.col_candidate_positions(c2, digit);
                            let rows3 = board.col_candidate_positions(c3, digit);
                            let rows4 = board.col_candidate_positions(c4, digit);
                            let mut union = std::collections::BTreeSet::new();
                            union.extend(rows1.iter());
                            union.extend(rows2.iter());
                            union.extend(rows3.iter());
                            union.extend(rows4.iter());
                            if rows1.len() <= FISH_LEN
                                && rows2.len() <= FISH_LEN
                                && rows3.len() <= FISH_LEN
                                && rows4.len() <= FISH_LEN
                                && union.len() == FISH_LEN
                            {
                                let changed = board::col_indices()
                                    .filter(|&c| c != c1 && c != c2 && c != c3 && c != c4)
                                    .flat_map(|c| union.iter().map(move |&r| (r, c)))
                                    .try_fold(false, |acc, (r, c)| {
                                        match board.eliminate_candidate(r, c, digit) {
                                            Some(true) => Ok(true),
                                            Some(false) => Ok(acc),
                                            None => {
                                                Err(SolverError::Contradiction { row: r, col: c })
                                            }
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
        }

        Ok(false)
    }
}
