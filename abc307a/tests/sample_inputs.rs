use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
1000 2000 3000 4000 5000 6000 7000 2000 3000 4000 5000 6000 7000 8000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "28000 35000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
14159 26535 89793 23846 26433 83279 50288 41971 69399 37510 58209 74944 59230 78164 6286 20899 86280 34825 34211 70679 82148
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "314333 419427 335328\n");
    assert!(output.stderr_str().is_empty());
}
