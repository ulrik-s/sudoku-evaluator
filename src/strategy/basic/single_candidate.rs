use crate::SolverError;
use crate::board::{Board, Digit};
use crate::strategy::{Strategy, StrategyKind};

pub struct SingleCandidate;

impl Strategy for SingleCandidate {
    fn kind(&self) -> StrategyKind {
        StrategyKind::SingleCandidate
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let mut res: Option<(usize, usize, Digit)> = None;
        board.try_for_each_cell_mut(|b, r, c| {
            let cand = b.candidates(r, c);
            if cand.len() == 1 {
                let digit = cand.into_iter().next().unwrap();
                res = Some((r, c, digit));
                Ok(true)
            } else {
                Ok(false)
            }
        })?;
        if let Some((r, c, d)) = res {
            board.set(r, c, d);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
