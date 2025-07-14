use crate::SolverError;
use crate::board::Board;

pub mod basic {
    pub mod box_line_reduction;
    pub mod hidden_pair;
    pub mod hidden_quad;
    pub mod hidden_single;
    pub mod hidden_triple;
    pub mod naked_pair;
    pub mod naked_quad;
    pub mod naked_triple;
    pub mod pointing_pair;
    pub mod single_candidate;
}

pub mod advanced {
    pub mod swordfish;
    pub mod x_wing;
    pub mod y_wing;
}

pub use advanced::*;
pub use basic::*;

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
    Swordfish,
}

pub trait Strategy {
    fn kind(&self) -> StrategyKind;
    fn apply(&self, board: &mut Board) -> Result<bool, SolverError>;
}
