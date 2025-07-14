use crate::strategy::{Strategy, StrategyKind};
use crate::board::{Board, Digit};
use crate::SolverError;
use std::collections::HashMap;

pub struct HiddenSingle;

impl Strategy for HiddenSingle {
    fn kind(&self) -> StrategyKind { StrategyKind::HiddenSingle }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        // rows
        for r in 0..9 {
            if let Some((c,d)) = find_hidden(board, (r,0..9), Scope::Row)? {
                board.set(r,c,d);
                return Ok(true);
            }
        }
        // cols
        for c in 0..9 {
            if let Some((r,d)) = find_hidden(board, (c,0..9), Scope::Col)? {
                board.set(r,c,d);
                return Ok(true);
            }
        }
        // boxes
        for br in 0..3 {
            for bc in 0..3 {
                if let Some((r,c,d)) = find_hidden_box(board, br*3, bc*3)? {
                    board.set(r,c,d);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

enum Scope { Row, Col }

fn find_hidden(board: &Board, index_range: (usize, std::ops::Range<usize>), scope: Scope)
    -> Result<Option<(usize, Digit)>, SolverError> {
    let (fixed, range) = index_range;
    let mut locs: HashMap<Digit, Vec<usize>> = HashMap::new();
    for var in range.clone() {
        let (r, c) = match scope {
            Scope::Row => (fixed, var),
            Scope::Col => (var, fixed),
        };
        if board.get(r,c).is_none() {
            for d in board.candidates(r,c) {
                locs.entry(d).or_default().push(var);
            }
        }
    }
    for (d, positions) in locs {
        if positions.len() == 1 {
            let var = positions[0];
            return Ok(Some((var, d)));
        }
    }
    Ok(None)
}

fn find_hidden_box(board: &Board, start_r: usize, start_c: usize)
    -> Result<Option<(usize, usize, Digit)>, SolverError> {
    let mut locs: HashMap<Digit, Vec<(usize, usize)>> = HashMap::new();
    for r in start_r..start_r+3 {
        for c in start_c..start_c+3 {
            if board.get(r,c).is_none() {
                for d in board.candidates(r,c) {
                    locs.entry(d).or_default().push((r,c));
                }
            }
        }
    }
    for (d, positions) in locs {
        if positions.len() == 1 {
            let (r,c) = positions[0];
            return Ok(Some((r,c,d)));
        }
    }
    Ok(None)
}
