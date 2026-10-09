use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("cross-platform CLI tool"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("setproxy"));
}

#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("当前代理状态"));
}

#[test]
fn test_set_proxy_outputs_structured() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.args(["127.0.0.1", "7890"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("SET"))
        .stdout(predicate::str::contains("http://127.0.0.1:7890"))
        .stdout(predicate::str::contains("socks5://127.0.0.1:7890"));
}

#[test]
fn test_clear_outputs_clear() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.arg("clear")
        .assert()
        .success()
        .stdout(predicate::str::is_match("^CLEAR\n?$").unwrap());
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::cargo_bin("_setproxy").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("请提供代理地址和端口"));
}
