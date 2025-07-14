use crate::board::Board;
use crate::strategy::{self, StrategyKind};
use crate::{Solver, SolverError};

/// Solver that progressively enables more advanced strategies.
pub struct ProgressiveSolver {
    basic: Vec<StrategyKind>,
}

impl Default for ProgressiveSolver {
    fn default() -> Self {
        Self {
            basic: vec![StrategyKind::SingleCandidate, StrategyKind::HiddenSingle],
        }
    }
}

impl ProgressiveSolver {
    /// Solve the puzzle by enabling strategies one by one.
    pub fn solve(&self, board: &mut Board) -> Result<Vec<StrategyKind>, SolverError> {
        let mut kinds = self.basic.clone();
        loop {
            Solver::new(strategies_from(&kinds)).reduce(board)?;
            if board.is_solved() {
                return Ok(kinds);
            }
            let snapshot = board.clone();
            let mut next = None;
            for &kind in &strategy::ALL_KINDS {
                if kinds.contains(&kind) {
                    continue;
                }
                let mut trial_board = snapshot.clone();
                let mut trial_kinds = kinds.clone();
                trial_kinds.push(kind);
                Solver::new(strategies_from(&trial_kinds)).reduce(&mut trial_board)?;
                if trial_board != snapshot {
                    next = Some(kind);
                    break;
                }
            }
            match next {
                Some(k) => kinds.push(k),
                None => return Err(SolverError::Unsolvable),
            }
        }
    }
}

fn strategies_from(kinds: &[StrategyKind]) -> Vec<Box<dyn strategy::Strategy>> {
    kinds
        .iter()
        .map(|&k| strategy::kind_to_strategy(k))
        .collect()
}
