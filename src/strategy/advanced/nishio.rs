use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};
use crate::{Solver, SolverError};

pub struct Nishio;

impl Strategy for Nishio {
    fn kind(&self) -> StrategyKind {
        StrategyKind::Nishio
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for r in board::row_indices() {
            for c in board::col_indices() {
                if board.get(r, c).is_some() {
                    continue;
                }
                let cands = board.candidates(r, c);
                if cands.len() <= 1 || cands.len() > 4 {
                    continue;
                }
                for d in cands.iter() {
                    let mut trial = board.clone();
                    trial.set(r, c, d);
                    let solver = Solver::without_nishio_and_forcing_chain();
                    if solver.solve(&mut trial).is_err() {
                        match board.eliminate_candidate(r, c, d) {
                            Some(true) => return Ok(true),
                            Some(false) => {}
                            None => return Err(SolverError::Contradiction { row: r, col: c }),
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}
