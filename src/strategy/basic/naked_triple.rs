use crate::SolverError;
use crate::board::{Board, CandidateSet, Unit};
use crate::strategy::{Strategy, StrategyKind};

pub struct NakedTriple;

impl Strategy for NakedTriple {
    fn kind(&self) -> StrategyKind {
        StrategyKind::NakedTriple
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for unit in Unit::all() {
            if search_unit(board, unit)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn search_unit(board: &mut Board, unit: Unit) -> Result<bool, SolverError> {
    let cells: Vec<((usize, usize), CandidateSet)> = board.unsolved_in_unit(unit).collect();
    for i in 0..cells.len() {
        for j in i + 1..cells.len() {
            for k in j + 1..cells.len() {
                let union = cells[i].1.union(cells[j].1).union(cells[k].1);
                if union.len() == 3
                    && cells[i].1.into_iter().all(|d| union.contains(d))
                    && cells[j].1.into_iter().all(|d| union.contains(d))
                    && cells[k].1.into_iter().all(|d| union.contains(d))
                {
                    let mut changed = false;
                    for (r, c) in board.unit_iter(unit) {
                        if (r, c) != cells[i].0
                            && (r, c) != cells[j].0
                            && (r, c) != cells[k].0
                            && board.get(r, c).is_none()
                        {
                            for d in &union {
                                match board.eliminate_candidate(r, c, d) {
                                    Some(true) => changed = true,
                                    None => {
                                        return Err(SolverError::Contradiction { row: r, col: c });
                                    }
                                    _ => {}
                                }
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
