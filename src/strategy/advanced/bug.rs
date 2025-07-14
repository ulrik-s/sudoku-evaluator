use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};

pub struct Bug;

impl Strategy for Bug {
    fn kind(&self) -> StrategyKind {
        StrategyKind::Bug
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let mut multi_cell = None;
        for r in board::row_indices() {
            for c in board::col_indices() {
                if board.get(r, c).is_none() {
                    let count = board.candidates(r, c).len();
                    if count > 2 {
                        if multi_cell.is_none() {
                            multi_cell = Some((r, c));
                        } else {
                            return Ok(false);
                        }
                    } else if count < 2 {
                        return Ok(false);
                    }
                }
            }
        }

        let (r, c) = match multi_cell {
            Some(pos) => pos,
            None => return Ok(false),
        };

        let mut digit_counts = [0usize; 10];
        for r0 in board::row_indices() {
            for c0 in board::col_indices() {
                if board.get(r0, c0).is_none() {
                    for d in board.candidates(r0, c0) {
                        digit_counts[d as usize] += 1;
                    }
                }
            }
        }

        let mut choice = None;
        for d in board.candidates(r, c) {
            if digit_counts[d as usize] == 3 {
                if choice.is_none() {
                    choice = Some(d);
                } else {
                    return Ok(false);
                }
            }
        }

        if let Some(d) = choice {
            board.set(r, c, d);
            return Ok(true);
        }

        Ok(false)
    }
}
