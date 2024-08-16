use assert_cmd::prelude::*;
use core::panic;
use std::process::Command;
use std::str;

#[test]
fn all_args() -> Result<(), Box<dyn std::error::Error>> {
    let args = ["first", "second", "-f", "987"];
    let output_bytes = Command::cargo_bin("arg_parse")?.args(args).output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf8 data from output_bytes"),
    };

    assert_eq!(output_str, "\"first\"\n\"second\"\n987\n");

    Ok(())
}

#[test]
fn first_and_flag() -> Result<(), Box<dyn std::error::Error>> {
    let args = ["first", "--flag", "6549841"];
    let output_bytes = Command::cargo_bin("arg_parse")?.args(args).output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf8 data from output_bytes"),
    };

    assert_eq!(output_str, "\"first\"\n\"6549841\n");

    Ok(())
}
