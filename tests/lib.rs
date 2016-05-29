use std::env;
use std::process::Command;

#[derive(Debug)]
pub struct NiceOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

fn run(args: Vec<&str>) -> NiceOutput {
    let mut directory = env::current_exe().unwrap();
            directory.pop(); // chop off exe name but leave "debug"
    let name = format!("to{}", env::consts::EXE_SUFFIX);
    let binary = directory.join(&name);

    let mut command = Command::new(binary);
            command.args(&args);
    let output = command.output().unwrap();
    let stdout = trim(output.stdout);
    let stderr = trim(output.stderr);

    return NiceOutput {
        status: output.status.code().unwrap(),
        stdout: stdout,
        stderr: stderr,
    };
}

fn trim(output: Vec<u8>) -> String {
    let string = String::from_utf8(output).unwrap();
    return String::from(string.trim());
}

#[test]
fn smoke_test() {
    let result = run(vec!["-h"]);
    assert_eq!(result.status, 0);
}

#[test]
fn version_test_long() {
    let result = run(vec!["--version"]);
    let version = env!("CARGO_PKG_VERSION");
    let expected = format!("to {}", version);

    assert_eq!(result.stdout, expected);
}

#[test]
fn version_test_short() {
    let result = run(vec!["-V"]);
    let version = env!("CARGO_PKG_VERSION");
    let expected = format!("to {}", version);

    assert_eq!(result.stdout, expected);
}
