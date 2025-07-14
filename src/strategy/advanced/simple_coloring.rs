use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct SimpleColoring;

impl Strategy for SimpleColoring {
    fn kind(&self) -> StrategyKind {
        StrategyKind::SimpleColoring
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        let mut changed = false;
        for digit in board::digits() {
            let mut adjacency: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
            for r in board::row_indices() {
                let pos = board.row_candidate_positions(r, digit);
                if pos.len() == 2 {
                    let mut it = pos.iter();
                    let c1 = it.next().unwrap();
                    let c2 = it.next().unwrap();
                    adjacency.entry((r, c1)).or_default().push((r, c2));
                    adjacency.entry((r, c2)).or_default().push((r, c1));
                }
            }
            for c in board::col_indices() {
                let pos = board.col_candidate_positions(c, digit);
                if pos.len() == 2 {
                    let mut it = pos.iter();
                    let r1 = it.next().unwrap();
                    let r2 = it.next().unwrap();
                    adjacency.entry((r1, c)).or_default().push((r2, c));
                    adjacency.entry((r2, c)).or_default().push((r1, c));
                }
            }
            for (br, bc) in board::box_indices() {
                let coords = board.candidate_coords(board::Unit::Box(br, bc), digit);
                if coords.len() == 2 {
                    let mut it = coords.iter();
                    let (r1, c1) = it.next().unwrap();
                    let (r2, c2) = it.next().unwrap();
                    adjacency.entry((r1, c1)).or_default().push((r2, c2));
                    adjacency.entry((r2, c2)).or_default().push((r1, c1));
                }
            }

            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            for &(r, c) in adjacency.keys() {
                if visited.contains(&(r, c)) {
                    continue;
                }
                let mut component = HashMap::new();
                let mut queue = VecDeque::new();
                component.insert((r, c), false);
                queue.push_back((r, c));
                visited.insert((r, c));
                while let Some((cr, cc)) = queue.pop_front() {
                    let color = *component.get(&(cr, cc)).unwrap();
                    if let Some(neigh) = adjacency.get(&(cr, cc)) {
                        for &(nr, nc) in neigh {
                            if let std::collections::hash_map::Entry::Vacant(e) =
                                component.entry((nr, nc))
                            {
                                e.insert(!color);
                                visited.insert((nr, nc));
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }

                let mut color_sets = [HashSet::new(), HashSet::new()];
                for (coord, &color) in &component {
                    color_sets[color as usize].insert(*coord);
                }

                let mut invalid = [false, false];
                for color in [false, true] {
                    for &(cr, cc) in &color_sets[color as usize] {
                        for peer in board.peer_coords(cr, cc) {
                            if color_sets[color as usize].contains(&peer) {
                                invalid[color as usize] = true;
                                break;
                            }
                        }
                        if invalid[color as usize] {
                            break;
                        }
                    }
                }

                for color in [false, true] {
                    if invalid[color as usize] {
                        for &(cr, cc) in &color_sets[color as usize] {
                            if let Some(res) = board.eliminate_candidate(cr, cc, digit) {
                                if res {
                                    changed = true;
                                }
                            } else {
                                return Err(SolverError::Contradiction { row: cr, col: cc });
                            }
                        }
                    }
                }

                for r0 in board::row_indices() {
                    for c0 in board::col_indices() {
                        if board.get(r0, c0).is_some() || !board.candidates(r0, c0).contains(digit)
                        {
                            continue;
                        }
                        if component.contains_key(&(r0, c0)) {
                            continue;
                        }
                        let mut seen = [false, false];
                        for peer in board.peer_coords(r0, c0) {
                            if let Some(&color) = component.get(&peer) {
                                seen[color as usize] = true;
                            }
                        }
                        if seen[0] && seen[1] {
                            if let Some(res) = board.eliminate_candidate(r0, c0, digit) {
                                if res {
                                    changed = true;
                                }
                            } else {
                                return Err(SolverError::Contradiction { row: r0, col: c0 });
                            }
                        }
                    }
                }
            }
        }
        Ok(changed)
    }
}
