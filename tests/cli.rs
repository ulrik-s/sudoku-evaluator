use std::process::Command;

#[test]
fn solve_cli_succeeds() {
    let puzzle =
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let output = Command::new(env!("CARGO_BIN_EXE_solve"))
        .arg(puzzle)
        .output()
        .expect("failed to run solve binary");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Solved with strategies"));
}
