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
fn test_status_lists_upper_case_variables() {
    setproxy()
        .arg("status")
        .env_remove("http_proxy")
        .env_remove("https_proxy")
        .env("HTTPS_PROXY", "http://10.0.0.1:3128")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "HTTPS_PROXY = http://10.0.0.1:3128",
        ));
}

#[test]
fn test_set_proxy_outputs_posix() {
    let assert = setproxy()
        .args(["--syntax", "posix", "127.0.0.1", "7890"])
        .assert()
        .success()
        // Messages go to stderr so stdout stays pure shell code
        .stdout(predicate::str::contains("echo").not())
        .stderr(predicate::str::contains("代理已启用"));

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    for line in [
        "export http_proxy='http://127.0.0.1:7890'",
        "export https_proxy='http://127.0.0.1:7890'",
        "export all_proxy='socks5://127.0.0.1:7890'",
        "export HTTP_PROXY='http://127.0.0.1:7890'",
        "export HTTPS_PROXY='http://127.0.0.1:7890'",
        "export ALL_PROXY='socks5://127.0.0.1:7890'",
    ] {
        assert!(stdout.contains(line), "missing {line:?} in:\n{stdout}");
    }
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
    let assert = setproxy()
        .args(["--syntax", "posix", "clear"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    for var in [
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
    ] {
        assert!(
            stdout.contains(&format!("unset {var}\n")),
            "missing unset {var}"
        );
    }

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
fn test_hook() {
    for shell in ["zsh", "bash"] {
        setproxy()
            .args(["hook", shell])
            .assert()
            .success()
            .stdout(predicate::str::contains("setproxy() {"))
            .stdout(predicate::str::contains("command setproxy --syntax posix"));
    }
    setproxy()
        .args(["hook", "pwsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("function setproxy"));

    // The startup file always names the shell
    setproxy().arg("hook").assert().failure();
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
        "export A=1\n\n# setproxy\neval \"$(setproxy hook zsh)\"\n"
    );
    fs::remove_dir_all(&home).unwrap();
}

#[test]
fn test_init_updates_previous_line_in_place() {
    let home = temp_home("update");
    let rc = home.join(".bashrc");
    fs::write(
        &rc,
        "export A=1\n# setproxy\neval \"$(setproxy init bash --print)\"\nexport B=2\n",
    )
    .unwrap();

    init_in(&home, "bash")
        .success()
        .stderr(predicate::str::contains("已更新"));

    assert_eq!(
        fs::read_to_string(&rc).unwrap(),
        "export A=1\n# setproxy\neval \"$(setproxy hook bash)\"\nexport B=2\n"
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

    assert_eq!(
        fs::read_to_string(zdotdir.join(".zshrc")).unwrap(),
        "# setproxy\neval \"$(setproxy hook zsh)\"\n"
    );
    assert!(!home.join(".zshrc").exists());
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
