extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("R2, L3")
        .stdout()
        .is("5")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("R2, R2, R2")
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("R5, L5, R5, R3")
        .stdout()
        .is("12")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("226")
        .unwrap();
}
