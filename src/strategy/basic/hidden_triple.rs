use crate::SolverError;
use crate::board::{Board, CandidateCoords, Unit};
use crate::strategy::{Strategy, StrategyKind};

pub struct HiddenTriple;

impl Strategy for HiddenTriple {
    fn kind(&self) -> StrategyKind {
        StrategyKind::HiddenTriple
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
    let mut positions: [CandidateCoords; 10] = [CandidateCoords::new(); 10];
    for d in 1..=9 {
        positions[d as usize] = board.candidate_coords(unit, d);
    }
    for d1 in 1..=7 {
        for d2 in d1 + 1..=8 {
            for d3 in d2 + 1..=9 {
                let mut union: Vec<(usize, usize)> = Vec::new();
                for pos in positions[d1 as usize].iter() {
                    if !union.contains(&pos) {
                        union.push(pos);
                    }
                }
                for pos in positions[d2 as usize].iter() {
                    if !union.contains(&pos) {
                        union.push(pos);
                    }
                }
                for pos in positions[d3 as usize].iter() {
                    if !union.contains(&pos) {
                        union.push(pos);
                    }
                }
                if union.len() == 3
                    && positions[d1 as usize].len() >= 1
                    && positions[d2 as usize].len() >= 1
                    && positions[d3 as usize].len() >= 1
                {
                    let mut changed = false;
                    for &(r, c) in &union {
                        for d in 1..=9 {
                            if d != d1 && d != d2 && d != d3 {
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
