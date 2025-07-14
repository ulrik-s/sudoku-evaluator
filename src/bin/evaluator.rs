use std::env;
use std::io::{self, Read};
use sudoku_evaluator::{
    Solver,
    board::Board,
    strategy::{Strategy, StrategyKind},
};

fn kind_to_strategy(kind: StrategyKind) -> Box<dyn Strategy> {
    match kind {
        StrategyKind::SingleCandidate => {
            Box::new(sudoku_evaluator::strategy::single_candidate::SingleCandidate)
        }
        StrategyKind::HiddenSingle => {
            Box::new(sudoku_evaluator::strategy::hidden_single::HiddenSingle)
        }
        StrategyKind::NakedPair => Box::new(sudoku_evaluator::strategy::naked_pair::NakedPair),
        StrategyKind::NakedTriple => {
            Box::new(sudoku_evaluator::strategy::naked_triple::NakedTriple)
        }
        StrategyKind::NakedQuad => Box::new(sudoku_evaluator::strategy::naked_quad::NakedQuad),
        StrategyKind::HiddenPair => Box::new(sudoku_evaluator::strategy::hidden_pair::HiddenPair),
        StrategyKind::HiddenTriple => {
            Box::new(sudoku_evaluator::strategy::hidden_triple::HiddenTriple)
        }
        StrategyKind::HiddenQuad => Box::new(sudoku_evaluator::strategy::hidden_quad::HiddenQuad),
        StrategyKind::PointingPair => {
            Box::new(sudoku_evaluator::strategy::pointing_pair::PointingPair)
        }
        StrategyKind::BoxLineReduction => {
            Box::new(sudoku_evaluator::strategy::box_line_reduction::BoxLineReduction)
        }
        StrategyKind::XWing => Box::new(sudoku_evaluator::strategy::x_wing::XWing),
        StrategyKind::YWing => Box::new(sudoku_evaluator::strategy::y_wing::YWing),
    }
}

const ALL_KINDS: [StrategyKind; 12] = [
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
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };
    let puzzle: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let base_board = Board::parse(&puzzle)?;

    let mut best: Option<(Vec<StrategyKind>, Board)> = None;

    for mask in 1usize..(1 << ALL_KINDS.len()) {
        let kinds: Vec<StrategyKind> = ALL_KINDS
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &k)| k)
            .collect();
        if let Some((ref best_kinds, _)) = best {
            if kinds.len() >= best_kinds.len() {
                continue;
            }
        }
        let strategies: Vec<Box<dyn Strategy>> =
            kinds.iter().map(|&k| kind_to_strategy(k)).collect();
        let solver = Solver::new(strategies);
        let mut board = base_board.clone();
        if solver.solve(&mut board).is_ok() {
            best = Some((kinds, board));
        }
    }

    if let Some((kinds, board)) = best {
        println!("Solved with strategies: {:?}", kinds);
        println!("{}", board);
    } else {
        println!("Puzzle could not be solved with available strategies");
    }

    Ok(())
}
