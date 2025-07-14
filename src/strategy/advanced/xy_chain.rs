use crate::SolverError;
use crate::board::Board;
use crate::strategy::{Strategy, StrategyKind};
use std::collections::{HashSet, VecDeque};

const PAIR_LEN: usize = 2;

pub struct XYChain;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    r: usize,
    c: usize,
    digit: u8,
    prev: u8,
}

impl Strategy for XYChain {
    fn kind(&self) -> StrategyKind {
        StrategyKind::XYChain
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let cells: Vec<_> = board.unsolved_cells().collect();
        for (r0, c0) in cells {
            let pivot = board.candidates(r0, c0);
            if pivot.len() != PAIR_LEN {
                continue;
            }

            let digits: Vec<u8> = pivot.iter().collect();
            let peers0 = board.peer_coords(r0, c0);

            for &start_digit in &digits {
                let other_digit = digits.iter().find(|&&d| d != start_digit).copied().unwrap();
                let start = Node {
                    r: r0,
                    c: c0,
                    digit: start_digit,
                    prev: other_digit,
                };
                let mut queue = VecDeque::new();
                queue.push_back(start);
                let mut visited: HashSet<(usize, usize, u8)> = HashSet::new();
                visited.insert((start.r, start.c, start.digit));

                while let Some(node) = queue.pop_front() {
                    let peers = board.peer_coords(node.r, node.c);
                    for &(nr, nc) in &peers {
                        let cand = board.candidates(nr, nc);
                        if cand.len() != PAIR_LEN || !cand.contains(node.digit) {
                            continue;
                        }
                        let next_digit = cand.iter().find(|&d| d != node.digit).unwrap();

                        if (nr, nc) != (r0, c0)
                            && next_digit == start_digit
                            && peers0.contains(&(nr, nc))
                        {
                            let peers1 = board.peer_coords(nr, nc);
                            let changed = peers0.iter().filter(|p| peers1.contains(p)).try_fold(
                                false,
                                |acc, &(rr, cc)| match board.eliminate_candidate(
                                    rr,
                                    cc,
                                    start_digit,
                                ) {
                                    Some(true) => Ok(true),
                                    Some(false) => Ok(acc),
                                    None => Err(SolverError::Contradiction { row: rr, col: cc }),
                                },
                            )?;
                            if changed {
                                return Ok(true);
                            }
                        }

                        if visited.insert((nr, nc, next_digit)) {
                            queue.push_back(Node {
                                r: nr,
                                c: nc,
                                digit: next_digit,
                                prev: node.digit,
                            });
                        }
                    }
                }
            }
        }

        Ok(false)
    }
}
