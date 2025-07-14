# Sudoku Evaluator

This crate provides a simple Sudoku solver that records which solving strategies were required. It includes common techniques such as Single Candidate, Hidden Single, Naked Pair, Naked Triple, Naked Quad, Hidden Pair, Hidden Triple, Hidden Quad, Pointing Pairs, Box-Line Reduction, X-Wing and Y-Wing.

```
use sudoku_evaluator::{board::Board, Solver};

let puzzle = "53467891267219534819834256785976142342685379171392485696153728428741963534528617.";
let mut board = Board::parse(puzzle).unwrap();
let solver = Solver::default();
let used = solver.solve(&mut board).unwrap();
assert!(board.is_solved());
println!("Strategies used: {:?}", used);
```

### Command line usage

Build and run the `solve` binary to solve a puzzle from the command line:

```
cargo run --bin solve -- "530070000600195000098000060800060003400803001700020006060000280000419005000080079"
```

The program prints the strategies that were required and the solved board.

Run tests with `cargo test`.

## Development

This project uses `rustfmt` and `clippy` for formatting and linting. Install them with:

```
rustup component add rustfmt clippy
```

Run the checks with:

```
cargo fmt -- --check
cargo clippy -- -D warnings
```

Then run the tests with:

```
cargo test -q
```

