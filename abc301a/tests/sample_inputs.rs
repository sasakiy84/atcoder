use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
TTAAT
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "T\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"6
ATTATA
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "T\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1
A
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "A\n");
    assert!(output.stderr_str().is_empty());
}
