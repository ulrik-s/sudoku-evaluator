use crate::SolverError;
use crate::board::{Board, Digit, Unit};
use crate::strategy::{Strategy, StrategyKind};
use std::collections::HashMap;

pub struct HiddenSingle;

impl Strategy for HiddenSingle {
    fn kind(&self) -> StrategyKind {
        StrategyKind::HiddenSingle
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for unit in Unit::all() {
            if let Some((r, c, d)) = find_hidden_unit(board, unit) {
                board.set(r, c, d);
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn find_hidden_unit(board: &Board, unit: Unit) -> Option<(usize, usize, Digit)> {
    let mut locs: HashMap<Digit, Vec<(usize, usize)>> = HashMap::new();
    board.for_each_in_unit(unit, |r, c, val| {
        if val.is_none() {
            for d in board.candidates(r, c) {
                locs.entry(d).or_default().push((r, c));
            }
        }
    });
    locs.into_iter().find_map(|(d, pos)| {
        (pos.len() == 1).then(|| {
            let (r, c) = pos[0];
            (r, c, d)
        })
    })
}
