use crate::SolverError;
use crate::board::{self, Board};
use crate::strategy::{Strategy, StrategyKind};

const PAIR_LEN: usize = 2;

pub struct XWing;

impl Strategy for XWing {
    fn kind(&self) -> StrategyKind {
        StrategyKind::XWing
    }

    fn apply(&self, board: &mut Board) -> Result<bool, SolverError> {
        // search rows
        if board::digits()
            .flat_map(|d| board::row_pairs().map(move |(r1, r2)| (d, r1, r2)))
            .find_map(|(digit, r1, r2)| {
                let cols1 = board.row_candidate_positions(r1, digit);
                let cols2 = board.row_candidate_positions(r2, digit);
                (cols1.len() == PAIR_LEN && cols2.len() == PAIR_LEN && cols1 == cols2).then(|| {
                    board::row_indices()
                        .filter(|&r| r != r1 && r != r2)
                        .flat_map(|r| cols1.iter().map(move |c| (r, c)))
                        .try_fold(false, |changed, (r, c)| {
                            match board.eliminate_candidate(r, c, digit) {
                                Some(true) => Ok(true),
                                Some(false) => Ok(changed),
                                None => Err(SolverError::Contradiction { row: r, col: c }),
                            }
                        })
                })
            })
            .transpose()?
            .unwrap_or(false)
        {
            return Ok(true);
        }

        // search columns
        if board::digits()
            .flat_map(|d| board::col_pairs().map(move |(c1, c2)| (d, c1, c2)))
            .find_map(|(digit, c1, c2)| {
                let rows1 = board.col_candidate_positions(c1, digit);
                let rows2 = board.col_candidate_positions(c2, digit);
                (rows1.len() == PAIR_LEN && rows2.len() == PAIR_LEN && rows1 == rows2).then(|| {
                    board::col_indices()
                        .filter(|&c| c != c1 && c != c2)
                        .flat_map(|c| rows1.iter().map(move |r| (r, c)))
                        .try_fold(false, |changed, (r, c)| {
                            match board.eliminate_candidate(r, c, digit) {
                                Some(true) => Ok(true),
                                Some(false) => Ok(changed),
                                None => Err(SolverError::Contradiction { row: r, col: c }),
                            }
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
