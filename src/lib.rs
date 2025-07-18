//! Sudoku solver that records which strategies were required.
//!
//! The [`Solver`] type runs through a list of strategies, attempting to apply
//! each one until the puzzle is solved. Errors are reported via [`SolverError`]
//! and [`BoardError`].

pub mod board;
pub use board::BoardError;
pub mod progressive;
pub mod strategy;
pub use progressive::ProgressiveSolver;

use board::Board;
use std::error::Error;
use strategy::{Strategy, StrategyKind};

/// Errors that can occur while solving a puzzle.
#[derive(Debug)]
pub enum SolverError {
    /// The board was found to contain a contradiction at the given cell.
    Contradiction { row: usize, col: usize },
    /// The initial board was invalid.
    InvalidBoard,
    /// The puzzle could not be solved with the available strategies.
    Unsolvable,
}

impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::Contradiction { row, col } => {
                write!(f, "contradiction at ({}, {})", row, col)
            }
            SolverError::InvalidBoard => write!(f, "board is invalid"),
            SolverError::Unsolvable => {
                write!(f, "puzzle cannot be solved with available strategies")
            }
        }
    }
}

impl Error for SolverError {}

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self { strategies }
    }

    pub fn with_default_strategies() -> Self {
        Self::default()
    }

    pub fn without_nishio() -> Self {
        Self::new(vec![
            Box::new(strategy::single_candidate::SingleCandidate),
            Box::new(strategy::hidden_single::HiddenSingle),
            Box::new(strategy::naked_pair::NakedPair),
            Box::new(strategy::naked_triple::NakedTriple),
            Box::new(strategy::naked_quad::NakedQuad),
            Box::new(strategy::hidden_pair::HiddenPair),
            Box::new(strategy::hidden_triple::HiddenTriple),
            Box::new(strategy::hidden_quad::HiddenQuad),
            Box::new(strategy::pointing_pair::PointingPair),
            Box::new(strategy::box_line_reduction::BoxLineReduction),
            Box::new(strategy::x_wing::XWing),
            Box::new(strategy::y_wing::YWing),
            Box::new(strategy::xyz_wing::XYZWing),
            Box::new(strategy::xy_wing::XYWing),
            Box::new(strategy::xy_chain::XYChain),
            Box::new(strategy::simple_coloring::SimpleColoring),
            Box::new(strategy::jellyfish::Jellyfish),
            Box::new(strategy::unique_rectangle::UniqueRectangle),
            Box::new(strategy::swordfish::Swordfish),
            Box::new(strategy::bug::Bug),
        ])
    }

    pub fn without_nishio_and_forcing_chain() -> Self {
        Self::new(vec![
            Box::new(strategy::single_candidate::SingleCandidate),
            Box::new(strategy::hidden_single::HiddenSingle),
            Box::new(strategy::naked_pair::NakedPair),
            Box::new(strategy::naked_triple::NakedTriple),
            Box::new(strategy::naked_quad::NakedQuad),
            Box::new(strategy::hidden_pair::HiddenPair),
            Box::new(strategy::hidden_triple::HiddenTriple),
            Box::new(strategy::hidden_quad::HiddenQuad),
            Box::new(strategy::pointing_pair::PointingPair),
            Box::new(strategy::box_line_reduction::BoxLineReduction),
            Box::new(strategy::x_wing::XWing),
            Box::new(strategy::y_wing::YWing),
            Box::new(strategy::xyz_wing::XYZWing),
            Box::new(strategy::xy_wing::XYWing),
            Box::new(strategy::xy_chain::XYChain),
            Box::new(strategy::simple_coloring::SimpleColoring),
            Box::new(strategy::jellyfish::Jellyfish),
            Box::new(strategy::unique_rectangle::UniqueRectangle),
            Box::new(strategy::swordfish::Swordfish),
            Box::new(strategy::bug::Bug),
        ])
    }

    fn apply_strategies(&self, board: &mut Board) -> Result<Vec<StrategyKind>, SolverError> {
        if !board.is_valid() {
            return Err(SolverError::InvalidBoard);
        }
        let mut used = Vec::new();
        loop {
            let mut progress = false;
            for strat in &self.strategies {
                let changed = strat.apply(board)?;
                if changed {
                    if !used.contains(&strat.kind()) {
                        used.push(strat.kind());
                    }
                    progress = true;
                    break;
                }
            }
            if !progress {
                break;
            }
        }
        Ok(used)
    }

    /// Attempt to fully solve the board.
    pub fn solve(&self, board: &mut Board) -> Result<Vec<StrategyKind>, SolverError> {
        let used = self.apply_strategies(board)?;
        if board.is_solved() {
            Ok(used)
        } else {
            Err(SolverError::Unsolvable)
        }
    }

    /// Apply strategies until no further progress can be made.
    pub fn reduce(&self, board: &mut Board) -> Result<Vec<StrategyKind>, SolverError> {
        self.apply_strategies(board)
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new(vec![
            Box::new(strategy::single_candidate::SingleCandidate),
            Box::new(strategy::hidden_single::HiddenSingle),
            Box::new(strategy::naked_pair::NakedPair),
            Box::new(strategy::naked_triple::NakedTriple),
            Box::new(strategy::naked_quad::NakedQuad),
            Box::new(strategy::hidden_pair::HiddenPair),
            Box::new(strategy::hidden_triple::HiddenTriple),
            Box::new(strategy::hidden_quad::HiddenQuad),
            Box::new(strategy::pointing_pair::PointingPair),
            Box::new(strategy::box_line_reduction::BoxLineReduction),
            Box::new(strategy::x_wing::XWing),
            Box::new(strategy::y_wing::YWing),
            Box::new(strategy::xyz_wing::XYZWing),
            Box::new(strategy::xy_wing::XYWing),
            Box::new(strategy::xy_chain::XYChain),
            Box::new(strategy::simple_coloring::SimpleColoring),
            Box::new(strategy::jellyfish::Jellyfish),
            Box::new(strategy::unique_rectangle::UniqueRectangle),
            Box::new(strategy::swordfish::Swordfish),
            Box::new(strategy::bug::Bug),
            Box::new(strategy::forcing_chain::ForcingChain),
            Box::new(strategy::nishio::Nishio),
        ])
    }
}
