use crate::SolverError;
use crate::board::{Board, Digit};
use crate::strategy::{Strategy, StrategyKind};

const PAIR_LEN: usize = 2;

pub struct YWing;

impl Strategy for YWing {
    fn kind(&self) -> StrategyKind {
        StrategyKind::YWing
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        if crate::board::row_indices()
            .flat_map(|r| crate::board::col_indices().map(move |c| (r, c)))
            .find_map(|(r, c)| {
                let pivot = board.candidates(r, c);
                if pivot.len() != PAIR_LEN {
                    return None;
                }
                let mut digits = pivot.iter();
                let d1: Digit = digits.next().unwrap();
                let d2: Digit = digits.next().unwrap();
                let peers = board.peer_coords(r, c);

                let first_wings: Vec<_> = peers
                    .iter()
                    .copied()
                    .filter_map(|(r1, c1)| {
                        let cand1 = board.candidates(r1, c1);
                        if cand1.len() != PAIR_LEN || !cand1.contains(d1) || cand1.contains(d2) {
                            return None;
                        }
                        cand1
                            .iter()
                            .find(|&d| d != d1)
                            .map(|other| ((r1, c1), other))
                    })
                    .collect();

                first_wings.into_iter().find_map(|((r1, c1), other)| {
                    let second_wings: Vec<_> = peers
                        .iter()
                        .copied()
                        .filter(|&(r2, c2)| (r2, c2) != (r1, c1))
                        .filter_map(|(r2, c2)| {
                            let cand2 = board.candidates(r2, c2);
                            if cand2.len() != PAIR_LEN || !cand2.contains(d2) || cand2.contains(d1)
                            {
                                return None;
                            }
                            cand2
                                .iter()
                                .find(|&d| d != d2)
                                .and_then(|o2| (o2 == other).then_some((r2, c2)))
                        })
                        .collect();

                    second_wings.into_iter().find_map(|(r2, c2)| {
                        let peers1 = board.peer_coords(r1, c1);
                        let peers2 = board.peer_coords(r2, c2);
                        peers1
                            .iter()
                            .filter(|p| peers2.contains(p))
                            .try_fold(false, |changed, &(rr, cc)| {
                                match board.eliminate_candidate(rr, cc, other) {
                                    Some(true) => Ok(true),
                                    Some(false) => Ok(changed),
                                    None => Err(SolverError::Contradiction { row: rr, col: cc }),
                                }
                            })
                            .map_or_else(
                                |e| Some(Err(e)),
                                |changed| if changed { Some(Ok(true)) } else { None },
                            )
                    })
                })
            })
            .transpose()?
            .unwrap_or(false)
        {
            return Ok(true);
        }
        Ok(false)
    }
}
