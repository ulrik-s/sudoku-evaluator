use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};

pub struct BoxLineReduction;

impl Strategy for BoxLineReduction {
    fn kind(&self) -> StrategyKind {
        StrategyKind::BoxLineReduction
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for br in 0..3 {
            for bc in 0..3 {
                for digit in 1..=9 {
                    let mut positions = Vec::new();
                    for r in br * 3..br * 3 + 3 {
                        for c in bc * 3..bc * 3 + 3 {
                            if board.get(r, c).is_none() && board.candidates(r, c).contains(digit) {
                                positions.push((r, c));
                            }
                        }
                    }
                    if positions.len() <= 1 {
                        continue;
                    }
                    let same_row = positions.iter().all(|&(r, _)| r == positions[0].0);
                    let same_col = positions.iter().all(|&(_, c)| c == positions[0].1);
                    if same_row {
                        let row = positions[0].0;
                        let mut changed = false;
                        for c in 0..9 {
                            if c < bc * 3 || c >= bc * 3 + 3 {
                                match board.eliminate_candidate(row, c, digit) {
                                    Some(true) => changed = true,
                                    None => return Err(SolverError::Contradiction { row, col: c }),
                                    _ => {}
                                }
                            }
                        }
                        if changed {
                            return Ok(true);
                        }
                    }
                    if same_col {
                        let col = positions[0].1;
                        let mut changed = false;
                        for r in 0..9 {
                            if r < br * 3 || r >= br * 3 + 3 {
                                match board.eliminate_candidate(r, col, digit) {
                                    Some(true) => changed = true,
                                    None => return Err(SolverError::Contradiction { row: r, col }),
                                    _ => {}
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
        Ok(false)
    }
}
