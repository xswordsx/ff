use assert_cmd::Command;
use std::string::String;

macro_rules! test_plain_type {
    ($func_name:ident, $val:expr) => {
        paste::item! {
            #[test]
            fn [< json_plain_type_ $func_name >] () {
                let mut cmd = Command::cargo_bin("ff").unwrap();
                cmd.write_stdin(stringify!($val)).assert()
                    .success()
                    .stdout(String::from(stringify!($val))+"\n");
            }
        }
    };
}

test_plain_type!(boolean, false);
test_plain_type!(null, null);
test_plain_type!(number, 1234);
test_plain_type!(string, "hello");

#[test]
fn empty_lines() {
    Command::cargo_bin("ff")
        .unwrap()
        .write_stdin("\n")
        .assert()
        .stdout("\n");
}

#[test]
fn sanity_check_no_color() {
    let input = r#"
    {
        "time": "2020-03-25T14:15:42.000Z",
        "message": "Hello from stdout",
        "severity": "info",
        "status_code": 200
    }
    "#;

    let expected_output = "14:15:42.000 [INFO ] Hello from stdout\n  status_code: 200\n";

    Command::cargo_bin("ff")
        .unwrap()
        .write_stdin(input.replace("\n", ""))
        .assert()
        .stdout(expected_output);
}

#[test]
fn sanity_check_color() {
    let input = r#"
    {
        "time": "1999-05-08T22:40:00.123Z",
        "message": "Hello from color-land",
        "severity": "error",
        "nested_1": {
            "nested_2": "correct!"
        }
    }
    "#
    .replace("\n", "");

    let mut expected_output = String::new();
    expected_output.push_str("22:40:00.123 [\x1B[31mERROR\x1B[0m] Hello from color-land");
    expected_output.push_str("\x1B[90m\n");
    expected_output.push_str("  nested_1:\n");
    expected_output.push_str("    nested_2: correct!\x1B[0m");
    expected_output.push_str("\n");

    Command::cargo_bin("ff")
        .unwrap()
        .env("CLICOLOR_FORCE", "1")
        .write_stdin(input)
        .assert()
        .stdout(expected_output);
}

#[test]
fn mixed_lines_input() {
    let input = [
        r#"{
            "time": "2022-03-13T16:15:24.059Z",
            "message": "First line - JSON",
            "severity": "debug"
        }"#
        .replace("\n", ""),
        String::from("This is just a plain-text line"),
        String::from(r#"{"message":"Second line"}"#),
    ]
    .join("\n");

    let expected_output = [
        "16:15:24.059 [DEBUG] First line - JSON",
        "This is just a plain-text line",
        "00:00:000.000 [UNKNOWN] Second line",
        "", // Command ends each line with a newline - accomodate the last one.
    ]
    .join("\n");

    Command::cargo_bin("ff")
        .unwrap()
        .write_stdin(input)
        .assert()
        .stdout(expected_output);
}

#[test]
fn component_extraction() {
    let input = String::from(
        r#"{
            "time": "2022-03-13T16:15:24.059Z",
            "message": "Some input line",
            "component": "COMPONENT",
            "severity": "debug"
        }"#,
    )
    .replace("\n", "");
    let expected_output = "16:15:24.059 [DEBUG] COMPONENT Some input line\n";

    Command::cargo_bin("ff")
        .unwrap()
        .write_stdin(input + "\n")
        .assert()
        .stdout(expected_output);
}
