use assert_cmd::prelude::*;
use core::panic;
use std::process::Command;
use std::{env, str};

#[test]
fn prints_data_in_a_table() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("table_of_chairs")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf8 data from output_bytes"),
    };

    Ok(())
}
