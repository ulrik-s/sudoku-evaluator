use std::io::{self, Read};
use sudoku_evaluator::{Solver, board::Board};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let puzzle: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut board = Board::from_str(&puzzle).unwrap();
    let solver = Solver::default();
    let res = solver.solve(&mut board);
    println!("res: {:?}", res);
    println!("{}", board);
}
