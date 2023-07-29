use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 4
110 90 120
100 80 120 10000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "110\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5 2
100000 100000 100000 100000 100000
100 200
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "201\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 2
100 100 100
80 120
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100\n");
    assert!(output.stderr_str().is_empty());
}
