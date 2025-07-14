use crate::strategy::{Strategy, StrategyKind};
use crate::board::Board;
use crate::SolverError;

pub struct SingleCandidate;

impl Strategy for SingleCandidate {
    fn kind(&self) -> StrategyKind {
        StrategyKind::SingleCandidate
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for r in 0..9 {
            for c in 0..9 {
                let cand = board.candidates(r,c);
                if cand.len() == 1 {
                    let digit = cand.into_iter().next().unwrap();
                    board.set(r,c,digit);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
