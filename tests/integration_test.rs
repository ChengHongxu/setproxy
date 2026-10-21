use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

fn setproxy() -> Command {
    Command::cargo_bin("setproxy").unwrap()
}

/// Empty home directory, so `init` never touches the real startup files
fn temp_home(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("setproxy-test-{}-{}", std::process::id(), name));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn init_in(home: &PathBuf, shell: &str) -> assert_cmd::assert::Assert {
    setproxy()
        .args(["init", shell])
        .env("HOME", home)
        .env_remove("ZDOTDIR")
        .assert()
}

#[test]
fn test_help_command() {
    setproxy()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("cross-platform CLI tool"));
}

#[test]
fn test_version_command() {
    setproxy()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("setproxy "));
}

#[test]
fn test_status_command() {
    setproxy()
        .arg("status")
        .assert()
        .success()
        .stderr(predicate::str::contains("当前代理状态"));
}

#[test]
fn test_set_proxy_outputs_posix() {
    setproxy()
        .args(["--syntax", "posix", "127.0.0.1", "7890"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "export http_proxy='http://127.0.0.1:7890'",
        ))
        .stdout(predicate::str::contains(
            "export all_proxy='socks5://127.0.0.1:7890'",
        ))
        // Messages go to stderr so stdout stays pure shell code
        .stdout(predicate::str::contains("echo").not())
        .stderr(predicate::str::contains("代理已启用"));
}

#[test]
fn test_set_proxy_outputs_powershell() {
    setproxy()
        .args(["--syntax", "powershell", "127.0.0.1", "7890"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "$env:https_proxy = 'http://127.0.0.1:7890'",
        ));
}

#[test]
fn test_clear_outputs_clear() {
    setproxy()
        .args(["--syntax", "posix", "clear"])
        .assert()
        .success()
        .stdout(predicate::str::contains("unset http_proxy"))
        .stdout(predicate::str::contains("unset https_proxy"))
        .stdout(predicate::str::contains("unset all_proxy"));

    setproxy()
        .args(["--syntax", "powershell", "c"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Remove-Item Env:all_proxy"));
}

#[test]
fn test_rejects_invalid_host() {
    setproxy()
        .args(["--syntax", "posix", "$(id)", "7890"])
        .assert()
        .failure()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_without_shell_function_prints_hint() {
    let home = temp_home("hint");
    for args in [&["127.0.0.1", "7890"][..], &["clear"], &["wsl"]] {
        setproxy()
            .args(args)
            .env("HOME", &home)
            .env_remove("ZDOTDIR")
            .assert()
            .failure()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::contains("setproxy init"));
    }
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn test_init_print() {
    for shell in ["zsh", "bash"] {
        setproxy()
            .args(["init", shell, "--print"])
            .assert()
            .success()
            .stdout(predicate::str::contains("setproxy() {"))
            .stdout(predicate::str::contains("command setproxy --syntax posix"));
    }
    setproxy()
        .args(["init", "pwsh", "--print"])
        .assert()
        .success()
        .stdout(predicate::str::contains("function setproxy"));

    setproxy().args(["init", "fish"]).assert().failure();
}

#[test]
fn test_init_appends_once_and_keeps_content() {
    let home = temp_home("append");
    let rc = home.join(".zshrc");
    fs::write(&rc, "export A=1").unwrap();

    init_in(&home, "zsh").success();
    init_in(&home, "zsh")
        .success()
        .stderr(predicate::str::contains("已配置"));

    assert_eq!(
        fs::read_to_string(&rc).unwrap(),
        "export A=1\n\n# setproxy\neval \"$(setproxy init zsh --print)\"\n"
    );
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn test_init_creates_missing_file_and_respects_zdotdir() {
    let home = temp_home("zdotdir");
    let zdotdir = home.join("zsh");

    setproxy()
        .args(["init", "zsh"])
        .env("HOME", &home)
        .env("ZDOTDIR", &zdotdir)
        .assert()
        .success();

    assert!(fs::read_to_string(zdotdir.join(".zshrc"))
        .unwrap()
        .starts_with("# setproxy\n"));
    assert!(!home.join(".zshrc").exists());
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn test_init_migrates_legacy_config() {
    let home = temp_home("migrate");
    let rc = home.join(".bashrc");
    fs::write(
        &rc,
        "export A=1\n\n# >>> setproxy initialized <<<\neval \"$(_setproxy shell-init)\"\n",
    )
    .unwrap();

    init_in(&home, "bash")
        .success()
        .stderr(predicate::str::contains("已移除旧版配置"));

    assert_eq!(
        fs::read_to_string(&rc).unwrap(),
        "export A=1\n\n# setproxy\neval \"$(setproxy init bash --print)\"\n"
    );
    fs::remove_dir_all(&home).unwrap();
}

/// The hint detects the calling shell from the process tree, so this needs
/// a Unix shell (zsh or bash) somewhere above the test process.
#[cfg(unix)]
#[test]
fn test_hint_when_configured_but_not_loaded() {
    let home = temp_home("configured");
    init_in(&home, "zsh").success();
    init_in(&home, "bash").success();

    setproxy()
        .args(["127.0.0.1", "7890"])
        .env("HOME", &home)
        .env_remove("ZDOTDIR")
        .assert()
        .failure()
        .stderr(predicate::str::contains("已配置"))
        .stderr(predicate::str::contains("source ~/"));
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn test_missing_arguments() {
    setproxy()
        .arg("--syntax=posix")
        .assert()
        .failure()
        .stderr(predicate::str::contains("请提供代理地址和端口"));
}
