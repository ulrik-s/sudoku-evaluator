use crate::SolverError;
use crate::board::{Board, Unit};
use crate::strategy::{Strategy, StrategyKind};

pub struct HiddenPair;

impl Strategy for HiddenPair {
    fn kind(&self) -> StrategyKind {
        StrategyKind::HiddenPair
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for unit in Unit::all() {
            if let Some(changed) = find_hidden_pair_unit(board, unit)? {
                if changed {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

fn find_hidden_pair_unit(board: &mut Board, unit: Unit) -> Result<Option<bool>, SolverError> {
    let mut positions: [Vec<(usize, usize)>; 10] = Default::default();
    for (r, c) in board.unit_iter(unit) {
        if board.get(r, c).is_none() {
            for d in board.candidates(r, c) {
                positions[d as usize].push((r, c));
            }
        }
    }
    for d1 in 1..=8 {
        for d2 in d1 + 1..=9 {
            if positions[d1 as usize].len() == 2 && positions[d1 as usize] == positions[d2 as usize]
            {
                let mut changed = false;
                for &(r, c) in &positions[d1 as usize] {
                    for d in 1..=9 {
                        if d != d1 && d != d2 {
                            match board.eliminate_candidate(r, c, d) {
                                Some(true) => changed = true,
                                None => return Err(SolverError::Contradiction { row: r, col: c }),
                                _ => {}
                            }
                        }
                    }
                }
                if changed {
                    return Ok(Some(true));
                }
            }
        }
    }
    Ok(None)
}
