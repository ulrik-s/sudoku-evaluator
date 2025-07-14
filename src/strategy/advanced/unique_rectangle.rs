use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};

pub struct UniqueRectangle;

impl Strategy for UniqueRectangle {
    fn kind(&self) -> StrategyKind {
        StrategyKind::UniqueRectangle
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for a in board::digits() {
            for b in board::digits() {
                if a >= b {
                    continue;
                }
                for (r1, r2) in board::row_pairs() {
                    for (c1, c2) in board::col_pairs() {
                        let coords = [(r1, c1), (r1, c2), (r2, c1), (r2, c2)];
                        let mut base_count = 0;
                        let mut extra_cell = None;
                        for &(r, c) in &coords {
                            let cands = board.candidates(r, c);
                            if cands.is_empty() {
                                base_count = 0;
                                break;
                            }
                            if cands == vec![a, b] || cands == vec![b, a] {
                                base_count += 1;
                            } else if cands.contains(a) && cands.contains(b) {
                                if extra_cell.is_none() {
                                    extra_cell = Some((r, c, cands));
                                } else {
                                    base_count = 0;
                                    break;
                                }
                            } else {
                                base_count = 0;
                                break;
                            }
                        }
                        if base_count == 3 {
                            if let Some((er, ec, cands)) = extra_cell {
                                let mut changed = false;
                                for d in cands.iter() {
                                    if d != a && d != b {
                                        if let Some(res) = board.eliminate_candidate(er, ec, d) {
                                            match res {
                                                true => changed = true,
                                                false => {}
                                            }
                                        } else {
                                            return Err(SolverError::Contradiction {
                                                row: er,
                                                col: ec,
                                            });
                                        }
                                    }
                                }
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
