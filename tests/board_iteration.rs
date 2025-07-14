use sudoku_evaluator::board::Board;
use sudoku_evaluator::board::Unit;

#[test]
fn for_each_visits_all_cells() {
    let board = Board::parse(&".".repeat(81)).unwrap();
    let mut count = 0;
    board.for_each_cell(|_, _, _| count += 1);
    assert_eq!(count, 81);
}

#[test]
fn try_for_each_stops_on_true() {
    let mut board = Board::parse(&".".repeat(81)).unwrap();
    let mut visited = 0;
    let res = board.try_for_each_cell_mut::<_, ()>(|_, _, _| {
        visited += 1;
        Ok(visited == 5)
    });
    assert!(res.unwrap());
    assert_eq!(visited, 5);
}

#[test]
fn try_for_each_cell_non_mut() {
    let board = Board::parse(&".".repeat(81)).unwrap();
    let mut visited = 0;
    let res = board.try_for_each_cell::<_, ()>(|_, _, _| {
        visited += 1;
        Ok(visited == 7)
    });
    assert!(res.unwrap());
    assert_eq!(visited, 7);
}

#[test]
fn for_each_cell_mut_counts() {
    let mut board = Board::parse(&".".repeat(81)).unwrap();
    let mut count = 0;
    board.for_each_cell_mut(|_, _, _| count += 1);
    assert_eq!(count, 81);
}

#[test]
fn unit_iteration_helpers() {
    let board = Board::parse(&".".repeat(81)).unwrap();
    let mut count = 0;
    board.for_each_in_unit(Unit::Row(0), |_, _, _| count += 1);
    assert_eq!(count, 9);

    let mut visited = 0;
    let mut board2 = board.clone();
    let res = board2.try_for_each_in_unit_mut::<_, ()>(Unit::Row(0), |_, _, _| {
        visited += 1;
        Ok(visited == 4)
    });
    assert!(res.unwrap());
    assert_eq!(visited, 4);
}

#[test]
fn box_iteration_helpers() {
    let board = Board::parse(&".".repeat(81)).unwrap();
    let mut count = 0;
    board.for_each_box(|_| count += 1);
    assert_eq!(count, 9);

    let mut combos = 0;
    board.for_each_box_digit(|_, _| combos += 1);
    assert_eq!(combos, 81);
}

#[test]
fn unsolved_cells_iterates_only_empty() {
    let puzzle = format!("1{}", ".".repeat(80));
    let board = Board::parse(&puzzle).unwrap();
    let coords: Vec<_> = board.unsolved_cells().collect();
    assert_eq!(coords.len(), 80);
    assert!(!coords.contains(&(0, 0)));
}
