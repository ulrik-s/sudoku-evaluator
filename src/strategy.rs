use crate::board::Board;
use crate::SolverError;

pub mod basic {
    pub mod single_candidate;
    pub mod hidden_single;
    pub mod naked_pair;
    pub mod naked_triple;
    pub mod naked_quad;
    pub mod hidden_pair;
    pub mod hidden_triple;
    pub mod hidden_quad;
    pub mod pointing_pair;
    pub mod box_line_reduction;
}

pub mod advanced {
    pub mod x_wing;
    pub mod y_wing;
}

pub use basic::*;
pub use advanced::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StrategyKind {
    SingleCandidate,
    HiddenSingle,
    NakedPair,
    NakedTriple,
    NakedQuad,
    HiddenPair,
    HiddenTriple,
    HiddenQuad,
    PointingPair,
    BoxLineReduction,
    XWing,
    YWing,
}

pub trait Strategy {
    fn kind(&self) -> StrategyKind;
    fn apply(&self, board: &mut Board) -> Result<bool, SolverError>;
}
