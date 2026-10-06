use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A cross-platform CLI tool"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("setproxy"));
}

#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("当前代理状态"));
}

#[test]
fn test_clear_command() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("clear")
        .assert()
        .success()
        .stdout(predicate::str::contains("unset http_proxy"));
}

#[test]
fn test_set_proxy_with_host_and_port() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.arg("192.168.1.1")
        .arg("8080")
        .assert()
        .success()
        .stdout(predicate::str::contains("export http_proxy"))
        .stdout(predicate::str::contains("http://192.168.1.1:8080"));
}

#[test]
fn test_missing_arguments() {
    let mut cmd = Command::cargo_bin("setproxy").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("请提供代理地址和端口"));
}
