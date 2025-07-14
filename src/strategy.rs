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
    pub mod bug;
    pub mod forcing_chain;
    pub mod jellyfish;
    pub mod nishio;
    pub mod simple_coloring;
    pub mod swordfish;
    pub mod unique_rectangle;
    pub mod x_wing;
    pub mod xy_chain;
    pub mod xy_wing;
    pub mod xyz_wing;
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
    XYZWing,
    XYWing,
    XYChain,
    SimpleColoring,
    Jellyfish,
    UniqueRectangle,
    Swordfish,
    Bug,
    ForcingChain,
    Nishio,
}

pub trait Strategy {
    fn kind(&self) -> StrategyKind;
    fn apply(&self, board: &mut Board) -> Result<bool, SolverError>;
}
