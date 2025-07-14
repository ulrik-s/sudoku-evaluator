use std::io::{self, Read};
use sudoku_evaluator::{Solver, board::Board};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };
    let puzzle: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut board = Board::parse(&puzzle)?;
    let solver = Solver::default();
    match solver.solve(&mut board) {
        Ok(strategies) => {
            println!("Solved with strategies: {:?}", strategies);
            println!("{}", board);
        }
        Err(e) => {
            println!("Failed to solve puzzle: {}", e);
            println!("{}", board);
        }
    }
    Ok(())
}
