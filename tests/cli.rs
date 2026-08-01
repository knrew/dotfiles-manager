use std::process::Command;

#[test]
fn cli_snapshots_match_the_cli_contract() {
    trycmd::TestCases::new().case("tests/cmd/*.toml");
}

#[test]
fn version_is_written_to_stdout_and_succeeds() {
    let output = Command::new(env!("CARGO_BIN_EXE_dotkoke"))
        .arg("--version")
        .output()
        .expect("dotkoke should run");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        format!("dotkoke {}\n", env!("CARGO_PKG_VERSION")).as_bytes()
    );
    assert!(output.stderr.is_empty());
}
