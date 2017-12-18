extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
ULL
RRDDD
LURDL
UUUUD",
        )
        .stdout()
        .is("5DB3")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("446A6")
        .unwrap();
}
