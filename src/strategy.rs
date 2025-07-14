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

/// All strategies in order from simplest to most advanced.
pub const ALL_KINDS: [StrategyKind; 22] = [
    StrategyKind::SingleCandidate,
    StrategyKind::HiddenSingle,
    StrategyKind::NakedPair,
    StrategyKind::NakedTriple,
    StrategyKind::NakedQuad,
    StrategyKind::HiddenPair,
    StrategyKind::HiddenTriple,
    StrategyKind::HiddenQuad,
    StrategyKind::PointingPair,
    StrategyKind::BoxLineReduction,
    StrategyKind::XWing,
    StrategyKind::YWing,
    StrategyKind::Swordfish,
    StrategyKind::Jellyfish,
    StrategyKind::UniqueRectangle,
    StrategyKind::XYZWing,
    StrategyKind::XYChain,
    StrategyKind::XYWing,
    StrategyKind::SimpleColoring,
    StrategyKind::Bug,
    StrategyKind::ForcingChain,
    StrategyKind::Nishio,
];

/// Create a boxed strategy instance for the given kind.
pub fn kind_to_strategy(kind: StrategyKind) -> Box<dyn Strategy> {
    match kind {
        StrategyKind::SingleCandidate => Box::new(single_candidate::SingleCandidate),
        StrategyKind::HiddenSingle => Box::new(hidden_single::HiddenSingle),
        StrategyKind::NakedPair => Box::new(naked_pair::NakedPair),
        StrategyKind::NakedTriple => Box::new(naked_triple::NakedTriple),
        StrategyKind::NakedQuad => Box::new(naked_quad::NakedQuad),
        StrategyKind::HiddenPair => Box::new(hidden_pair::HiddenPair),
        StrategyKind::HiddenTriple => Box::new(hidden_triple::HiddenTriple),
        StrategyKind::HiddenQuad => Box::new(hidden_quad::HiddenQuad),
        StrategyKind::PointingPair => Box::new(pointing_pair::PointingPair),
        StrategyKind::BoxLineReduction => Box::new(box_line_reduction::BoxLineReduction),
        StrategyKind::XWing => Box::new(x_wing::XWing),
        StrategyKind::YWing => Box::new(y_wing::YWing),
        StrategyKind::Swordfish => Box::new(swordfish::Swordfish),
        StrategyKind::Jellyfish => Box::new(jellyfish::Jellyfish),
        StrategyKind::UniqueRectangle => Box::new(unique_rectangle::UniqueRectangle),
        StrategyKind::XYZWing => Box::new(xyz_wing::XYZWing),
        StrategyKind::XYChain => Box::new(xy_chain::XYChain),
        StrategyKind::XYWing => Box::new(xy_wing::XYWing),
        StrategyKind::SimpleColoring => Box::new(simple_coloring::SimpleColoring),
        StrategyKind::Bug => Box::new(bug::Bug),
        StrategyKind::ForcingChain => Box::new(forcing_chain::ForcingChain),
        StrategyKind::Nishio => Box::new(nishio::Nishio),
    }
}
