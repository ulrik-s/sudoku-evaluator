use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};
use crate::{Solver, SolverError};

pub struct ForcingChain;

impl Strategy for ForcingChain {
    fn kind(&self) -> StrategyKind {
        StrategyKind::ForcingChain
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let mut target = None;
        let mut best_len = 10;
        for (r, c) in board.unsolved_cells() {
            let cands = board.candidates(r, c);
            let len = cands.len();
            if len > 1 && len < best_len {
                best_len = len;
                target = Some((r, c, cands));
                if len == 2 {
                    break;
                }
            }
        }

        let (r, c, cands) = match target {
            Some(t) => t,
            None => return Ok(false),
        };

        let mut solution = None;
        for d in cands.iter() {
            let mut trial = board.clone();
            trial.set(r, c, d);
            let solver = Solver::without_nishio_and_forcing_chain();
            if solver.solve(&mut trial).is_ok() {
                if solution.is_some() {
                    return Ok(false);
                }
                solution = Some(d);
            }
        }
        if let Some(d) = solution {
            board.set(r, c, d);
            return Ok(true);
        }
        Ok(false)
    }
}
