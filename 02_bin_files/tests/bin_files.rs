use assert_cmd::prelude::*;
use std::process::Command;
use std::{panic, str};

#[test]
fn adds_numbers_to_6() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("bin_files")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf8 data from output_bytes"),
    };

    assert_eq!(output_str, "6\n");

    Ok(())
}
