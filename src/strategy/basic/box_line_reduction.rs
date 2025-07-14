use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};

pub struct BoxLineReduction;

impl Strategy for BoxLineReduction {
    fn kind(&self) -> StrategyKind {
        StrategyKind::BoxLineReduction
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        board.try_for_each_box_digit_mut(|b, unit, digit| {
            let mut positions = Vec::new();
            b.for_each_in_unit(unit, |r, c, val| {
                if val.is_none() && b.candidates(r, c).contains(digit) {
                    positions.push((r, c));
                }
            });
            if positions.len() <= 1 {
                return Ok(false);
            }

            let same_row = positions.iter().all(|&(r, _)| r == positions[0].0);
            let same_col = positions.iter().all(|&(_, c)| c == positions[0].1);

            if same_row {
                let row = positions[0].0;
                let mut changed = false;
                for c in 0..9 {
                    if !unit.contains(row, c) {
                        match b.eliminate_candidate(row, c, digit) {
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
                    if !unit.contains(r, col) {
                        match b.eliminate_candidate(r, col, digit) {
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

            Ok(false)
        })
    }
}
