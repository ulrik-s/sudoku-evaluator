use crate::strategy::{Strategy, StrategyKind};
use crate::board::{Board, CandidateSet, Unit};
use crate::SolverError;

pub struct NakedQuad;

impl Strategy for NakedQuad {
    fn kind(&self) -> StrategyKind { StrategyKind::NakedQuad }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for unit in Unit::all() {
            if search_unit(board, unit)? { return Ok(true); }
        }
        Ok(false)
    }
}

fn search_unit(board: &mut Board, unit: Unit) -> Result<bool, SolverError> {
    let cells: Vec<((usize, usize), CandidateSet)> = board.unsolved_in_unit(unit).collect();
    for a in 0..cells.len() {
        for b in a + 1..cells.len() {
            for c in b + 1..cells.len() {
                for d in c + 1..cells.len() {
                    let union = cells[a].1.union(cells[b].1).union(cells[c].1).union(cells[d].1);
                    if union.len() == 4
                        && cells[a].1.iter().all(|v| union.contains(v))
                        && cells[b].1.iter().all(|v| union.contains(v))
                        && cells[c].1.iter().all(|v| union.contains(v))
                        && cells[d].1.iter().all(|v| union.contains(v))
                    {
                        let mut changed = false;
                        for (r, cidx) in board.unit_iter(unit) {
                            if (r, cidx) != cells[a].0
                                && (r, cidx) != cells[b].0
                                && (r, cidx) != cells[c].0
                                && (r, cidx) != cells[d].0
                                && board.get(r, cidx).is_none()
                            {
                                for digit in &union {
                                    match board.eliminate_candidate(r, cidx, digit) {
                                        Some(true) => changed = true,
                                        None => return Err(SolverError::Contradiction { row: r, col: cidx }),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if changed { return Ok(true); }
                    }
                }
            }
        }
    }
    Ok(false)
}
