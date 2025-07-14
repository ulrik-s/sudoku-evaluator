use sudoku_evaluator::{ProgressiveSolver, SolverError, board::Board, strategy::StrategyKind};

#[test]
fn progressive_solver_solves_puzzle() {
    let puzzle =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut board = Board::parse(puzzle).unwrap();
    let solver = ProgressiveSolver::default();
    let kinds = solver.solve(&mut board).unwrap();
    assert!(board.is_solved());
    assert_eq!(
        kinds,
        vec![StrategyKind::SingleCandidate, StrategyKind::HiddenSingle]
    );
}

#[test]
fn progressive_solver_unsolvable() {
    let puzzle =
        "000982000035100870800300059090015000002000600000620040900201003013006520000700000";
    let mut board = Board::parse(puzzle).unwrap();
    let solver = ProgressiveSolver::default();
    let err = solver.solve(&mut board).unwrap_err();
    assert!(matches!(err, SolverError::Unsolvable));
}
