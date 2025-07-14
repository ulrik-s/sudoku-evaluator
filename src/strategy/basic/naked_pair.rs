use crate::SolverError;
use crate::board::{Board, CandidateSet, Unit};
use crate::strategy::{Strategy, StrategyKind};

pub struct NakedPair;

impl Strategy for NakedPair {
    fn kind(&self) -> StrategyKind {
        StrategyKind::NakedPair
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
    let cells: Vec<((usize, usize), CandidateSet)> = board
        .unit_iter(unit)
        .filter(|&(r, c)| board.get(r, c).is_none())
        .map(|(r, c)| ((r, c), board.candidates(r, c)))
        .filter(|(_, cand)| cand.len() == 2)
        .collect();

    for i in 0..cells.len() {
        for j in i + 1..cells.len() {
            if cells[i].1 == cells[j].1 {
                let digits = cells[i].1;
                // ensure this pair occurs exactly in two cells
                if cells.iter().filter(|c| c.1 == digits).count() != 2 {
                    continue;
                }
                let mut changed = false;
                for (rr, cc) in board.unit_iter(unit) {
                    if (rr, cc) != cells[i].0
                        && (rr, cc) != cells[j].0
                        && board.get(rr, cc).is_none()
                    {
                        let cell_cands = board.candidates(rr, cc);
                        if cell_cands.len() == digits.len() && cell_cands == digits {
                            continue;
                        }
                        for d in &digits {
                            match board.eliminate_candidate(rr, cc, d) {
                                Some(true) => changed = true,
                                None => {
                                    return Err(SolverError::Contradiction { row: rr, col: cc });
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
    Ok(false)
}
