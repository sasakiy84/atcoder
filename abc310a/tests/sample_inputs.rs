use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 100 50
60 20 40
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "70\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3 100 50
60000 20000 40000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100\n");
    assert!(output.stderr_str().is_empty());
}
