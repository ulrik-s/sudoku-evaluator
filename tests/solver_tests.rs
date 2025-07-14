use sudoku_evaluator::{board::Board, Solver, strategy::{StrategyKind, Strategy}};

#[test]
fn already_solved() {
    let puzzle = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let mut board = Board::from_str(puzzle).unwrap();
    let solver = Solver::default();
    let used = solver.solve(&mut board).unwrap();
    assert!(used.is_empty());
}

#[test]
fn single_candidate_only() {
    let puzzle = "53467891267219534819834256785976142342685379171392485696153728428741963534528617.";
    let mut board = Board::from_str(puzzle).unwrap();
    let solver = Solver::default();
    let used = solver.solve(&mut board).unwrap();
    assert_eq!(used, vec![StrategyKind::SingleCandidate]);
    assert!(board.is_solved());
}

#[test]
fn invalid_board() {
    let puzzle = format!("11{}", ".".repeat(79)); // duplicate 1 in row
    let mut board = Board::from_str(&puzzle).unwrap();
    let solver = Solver::default();
    let err = solver.solve(&mut board).unwrap_err();
    assert!(matches!(err, sudoku_evaluator::SolverError::InvalidBoard));
}

#[test]
fn solve_easy_puzzle() {
    let puzzle = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let mut board = Board::from_str(puzzle).unwrap();
    let solver = Solver::default();
    let used = solver.solve(&mut board).unwrap();
    assert!(board.is_solved());
    assert!(!used.is_empty());
}

#[test]
fn solve_harder_puzzle() {
    let puzzle = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    let mut board = Board::from_str(puzzle).unwrap();
    let solver = Solver::default();
    let used = solver.solve(&mut board).unwrap();
    assert!(board.is_solved());
    assert!(!used.is_empty());
}

#[test]
fn naked_pair_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    // setup row 0 naked pair in c0 and c1, candidate {1,2}
    for d in 3..=9 {
        board.eliminate_candidate(0, 0, d);
        board.eliminate_candidate(0, 1, d);
    }
    // cell c2 has {1,2,3}
    for d in 4..=9 {
        board.eliminate_candidate(0, 2, d);
    }
    let strat = sudoku_evaluator::strategy::naked_pair::NakedPair;
    assert!(strat.apply(&mut board).unwrap());
    assert_eq!(board.candidates(0, 2), vec![3]);
}

#[test]
fn box_line_reduction_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    // in box (0,0), digit 1 only appears in row 0
    for d in 3..=9 {
        board.eliminate_candidate(0, 0, d);
        board.eliminate_candidate(0, 1, d);
    }
    for r in 1..=2 {
        for c in 0..=2 {
            board.eliminate_candidate(r, c, 1);
        }
    }
    board.eliminate_candidate(0, 3, 2); // leave candidate 1 in c3
    for d in [2,3,5,6,7,8,9] {
        board.eliminate_candidate(0, 3, d);
    }
    for d in 3..=9 {
        board.eliminate_candidate(0, 4, d);
    }
    let strat = sudoku_evaluator::strategy::box_line_reduction::BoxLineReduction;
    assert!(strat.apply(&mut board).unwrap());
    assert!(!board.candidates(0, 3).contains(1));
}

#[test]
fn solve_very_hard_puzzle() {
    let puzzle = "..467.....7..9...8.9..4.5.7.5...1.234.6..37...........9...372.4.8.4.9.3...5.8...9";
    let mut board = Board::from_str(puzzle).unwrap();
    let solver = Solver::default();
    let used = solver.solve(&mut board).unwrap();
    assert!(board.is_solved());
    assert!(!used.is_empty());
}

#[test]
fn x_wing_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    for r in 0..2 {
        for c in 0..9 {
            if c != 0 && c != 2 {
                board.eliminate_candidate(r, c, 1);
            }
        }
    }
    let strat = sudoku_evaluator::strategy::x_wing::XWing;
    assert!(strat.apply(&mut board).unwrap());
    for r in 2..9 {
        assert!(!board.candidates(r, 0).contains(1));
        assert!(!board.candidates(r, 2).contains(1));
    }
}

#[test]
fn y_wing_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    for d in 1..=9 {
        if d != 1 && d != 2 {
            board.eliminate_candidate(0, 0, d);
        }
        if d != 1 && d != 3 {
            board.eliminate_candidate(0, 1, d);
        }
        if d != 2 && d != 3 {
            board.eliminate_candidate(1, 0, d);
        }
        if d >= 4 {
            board.eliminate_candidate(1, 1, d);
        }
    }
    let strat = sudoku_evaluator::strategy::y_wing::YWing;
    assert!(strat.apply(&mut board).unwrap());
    assert!(!board.candidates(1, 1).contains(3));
}

#[test]
fn hidden_pair_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    for c in 0..9 {
        for d in 1..=9 {
            if c == 0 || c == 1 {
                if d >= 4 { board.eliminate_candidate(0, c, d); }
            } else {
                if d == 1 || d == 2 { board.eliminate_candidate(0, c, d); }
                if d >= 6 { board.eliminate_candidate(0, c, d); }
            }
        }
    }
    let strat = sudoku_evaluator::strategy::hidden_pair::HiddenPair;
    assert!(strat.apply(&mut board).unwrap());
    assert_eq!(board.candidates(0,0), vec![1,2]);
    assert_eq!(board.candidates(0,1), vec![1,2]);
}

#[test]
fn naked_triple_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    for c in 0..3 {
        for d in 4..=9 { board.eliminate_candidate(0, c, d); }
    }
    for d in 5..=9 { board.eliminate_candidate(0, 3, d); }
    let strat = sudoku_evaluator::strategy::naked_triple::NakedTriple;
    assert!(strat.apply(&mut board).unwrap());
    assert_eq!(board.candidates(0,3), vec![4]);
}



#[test]
fn hidden_triple_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    for c in 3..9 {
        board.eliminate_candidate(0,c,1);
        board.eliminate_candidate(0,c,2);
        board.eliminate_candidate(0,c,3);
    }
    for d in 5..=9 { board.eliminate_candidate(0,0,d); }
    for d in 4..=9 { if d!=5 { board.eliminate_candidate(0,1,d); } }
    for d in 4..=9 { if d!=6 { board.eliminate_candidate(0,2,d); } }
    let strat = sudoku_evaluator::strategy::hidden_triple::HiddenTriple;
    assert!(strat.apply(&mut board).unwrap());
    assert_eq!(board.candidates(0,0), vec![1,2,3]);
    assert_eq!(board.candidates(0,1), vec![1,2,3]);
    assert_eq!(board.candidates(0,2), vec![1,2,3]);
}

#[test]
fn pointing_pair_strategy() {
    let mut board = Board::from_str(&".".repeat(81)).unwrap();
    // digit 1 only in cells (0,0) and (0,1) of box 0
    for r in 0..3 {
        for c in 0..3 {
            if !(r==0 && (c==0 || c==1)) {
                board.eliminate_candidate(r,c,1);
            }
        }
    }
    let strat = sudoku_evaluator::strategy::pointing_pair::PointingPair;
    assert!(strat.apply(&mut board).unwrap());
    for c in 2..9 {
        if c>=3 {
            assert!(!board.candidates(0,c).contains(1));
        }
    }
}
