use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};

const TRIPLE_LEN: usize = 3;

pub struct XYZWing;

impl Strategy for XYZWing {
    fn kind(&self) -> StrategyKind {
        StrategyKind::XYZWing
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        for r in crate::board::row_indices() {
            for c in crate::board::col_indices() {
                let pivot = board.candidates(r, c);
                if pivot.len() != TRIPLE_LEN {
                    continue;
                }
                let digits: Vec<u8> = pivot.iter().collect();
                let peers = board.peer_coords(r, c);

                for i in 0..TRIPLE_LEN {
                    let z = digits[i];
                    let x = digits[(i + 1) % TRIPLE_LEN];
                    let y = digits[(i + 2) % TRIPLE_LEN];
                    let w1s: Vec<_> = peers
                        .iter()
                        .copied()
                        .filter(|&(r1, c1)| board.candidates(r1, c1) == vec![x, z])
                        .collect();
                    let w2s: Vec<_> = peers
                        .iter()
                        .copied()
                        .filter(|&(r2, c2)| board.candidates(r2, c2) == vec![y, z])
                        .collect();
                    for (r1, c1) in &w1s {
                        for (r2, c2) in &w2s {
                            if (r1, c1) == (r2, c2) {
                                continue;
                            }
                            let peers1 = board.peer_coords(*r1, *c1);
                            let peers2 = board.peer_coords(*r2, *c2);
                            let changed = peers1
                                .iter()
                                .filter(|p| peers2.contains(p) && peers.contains(p))
                                .try_fold(false, |acc, &(rr, cc)| {
                                    match board.eliminate_candidate(rr, cc, z) {
                                        Some(true) => Ok(true),
                                        Some(false) => Ok(acc),
                                        None => {
                                            Err(SolverError::Contradiction { row: rr, col: cc })
                                        }
                                    }
                                })?;
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
}
