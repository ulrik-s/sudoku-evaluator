use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};

const PAIR_LEN: usize = 2;

pub struct XYWing;

impl Strategy for XYWing {
    fn kind(&self) -> StrategyKind {
        StrategyKind::XYWing
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let cells: Vec<_> = board.unsolved_cells().collect();
        for (r, c) in cells {
            let pivot = board.candidates(r, c);
            if pivot.len() != PAIR_LEN {
                continue;
            }

            let mut digits = pivot.iter();
            let a = digits.next().unwrap();
            let b = digits.next().unwrap();
            let peers = board.peer_coords(r, c);

            for &(r1, c1) in &peers {
                let cand1 = board.candidates(r1, c1);
                if cand1.len() != PAIR_LEN || !cand1.contains(a) || cand1.contains(b) {
                    continue;
                }
                let z = cand1.iter().find(|&d| d != a).unwrap();

                for &(r2, c2) in &peers {
                    if (r2, c2) == (r1, c1) {
                        continue;
                    }
                    let cand2 = board.candidates(r2, c2);
                    if cand2.len() != PAIR_LEN || !cand2.contains(b) || cand2.contains(a) {
                        continue;
                    }
                    if !cand2.contains(z) {
                        continue;
                    }

                    let peers1 = board.peer_coords(r1, c1);
                    let peers2 = board.peer_coords(r2, c2);
                    let changed = peers1.iter().filter(|p| peers2.contains(p)).try_fold(
                        false,
                        |acc, &(rr, cc)| match board.eliminate_candidate(rr, cc, z) {
                            Some(true) => Ok(true),
                            Some(false) => Ok(acc),
                            None => Err(SolverError::Contradiction { row: rr, col: cc }),
                        },
                    )?;
                    if changed {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}
