use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};

pub struct HiddenQuad;

impl Strategy for HiddenQuad {
    fn kind(&self) -> StrategyKind {
        StrategyKind::HiddenQuad
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for r in 0..9 {
            if search_row(board, r)? {
                return Ok(true);
            }
        }
        for c in 0..9 {
            if search_col(board, c)? {
                return Ok(true);
            }
        }
        for br in 0..3 {
            for bc in 0..3 {
                if search_box(board, br * 3, bc * 3)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

fn search_row(board: &mut Board, r: usize) -> Result<bool, SolverError> {
    let mut positions: [Vec<usize>; 10] = Default::default();
    for c in 0..9 {
        if board.get(r, c).is_none() {
            for d in board.candidates(r, c) {
                positions[d as usize].push(c);
            }
        }
    }
    apply_hidden(positions, |idx| (r, idx), board)
}

fn search_col(board: &mut Board, c: usize) -> Result<bool, SolverError> {
    let mut positions: [Vec<usize>; 10] = Default::default();
    for r in 0..9 {
        if board.get(r, c).is_none() {
            for d in board.candidates(r, c) {
                positions[d as usize].push(r);
            }
        }
    }
    apply_hidden(positions, |idx| (idx, c), board)
}

fn search_box(board: &mut Board, start_r: usize, start_c: usize) -> Result<bool, SolverError> {
    let mut positions: [Vec<(usize, usize)>; 10] = Default::default();
    for r in start_r..start_r + 3 {
        for c in start_c..start_c + 3 {
            if board.get(r, c).is_none() {
                for d in board.candidates(r, c) {
                    positions[d as usize].push((r, c));
                }
            }
        }
    }
    for d1 in 1..=6 {
        for d2 in d1 + 1..=7 {
            for d3 in d2 + 1..=8 {
                for d4 in d3 + 1..=9 {
                    let union: Vec<(usize, usize)> = positions[d1 as usize]
                        .iter()
                        .chain(&positions[d2 as usize])
                        .chain(&positions[d3 as usize])
                        .chain(&positions[d4 as usize])
                        .copied()
                        .collect();
                    let mut uniq = union.clone();
                    uniq.sort_unstable();
                    uniq.dedup();
                    if uniq.len() == 4 {
                        let mut changed = false;
                        for &(r, c) in &uniq {
                            for d in 1..=9 {
                                if d != d1 && d != d2 && d != d3 && d != d4 {
                                    match board.eliminate_candidate(r, c, d) {
                                        Some(true) => changed = true,
                                        None => {
                                            return Err(SolverError::Contradiction {
                                                row: r,
                                                col: c,
                                            });
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
    }
    Ok(false)
}

fn apply_hidden<F>(
    positions: [Vec<usize>; 10],
    idx_to_coord: F,
    board: &mut Board,
) -> Result<bool, SolverError>
where
    F: Fn(usize) -> (usize, usize),
{
    for d1 in 1..=6 {
        for d2 in d1 + 1..=7 {
            for d3 in d2 + 1..=8 {
                for d4 in d3 + 1..=9 {
                    let pos1 = &positions[d1 as usize];
                    let pos2 = &positions[d2 as usize];
                    let pos3 = &positions[d3 as usize];
                    let pos4 = &positions[d4 as usize];
                    let mut union: Vec<usize> = pos1.iter().copied().collect();
                    for &p in pos2 {
                        if !union.contains(&p) {
                            union.push(p);
                        }
                    }
                    for &p in pos3 {
                        if !union.contains(&p) {
                            union.push(p);
                        }
                    }
                    for &p in pos4 {
                        if !union.contains(&p) {
                            union.push(p);
                        }
                    }
                    if union.len() == 4 {
                        let mut changed = false;
                        for &idx in &union {
                            let (r, c) = idx_to_coord(idx);
                            for d in 1..=9 {
                                if d != d1 && d != d2 && d != d3 && d != d4 {
                                    match board.eliminate_candidate(r, c, d) {
                                        Some(true) => changed = true,
                                        None => {
                                            return Err(SolverError::Contradiction {
                                                row: r,
                                                col: c,
                                            });
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
    }
    Ok(false)
}
