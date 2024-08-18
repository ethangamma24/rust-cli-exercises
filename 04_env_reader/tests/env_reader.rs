use assert_cmd::prelude::*;
use core::panic;
use std::process::Command;
use std::{env, str};

#[test]
fn var_2_set() -> Result<(), Box<dyn std::error::Error>> {
    let key = "MY_VAR_2";
    unsafe {
        env::set_var(key, "friends!");
        let output_bytes = Command::cargo_bin("env_reader")?.output()?.stdout;

        let output_str = match str::from_utf8(&output_bytes) {
            Ok(val) => val,
            Err(_) => panic!("Got non utf8 data from output_bytes"),
        };

        assert_eq!(output_str, "hello!!\n\"friends!\"\n");
        env::remove_var(key);
    }

    Ok(())
}

#[test]
fn var_2_unset() -> Result<(), Box<dyn std::error::Error>> {
    let output_bytes = Command::cargo_bin("env_reader")?.output()?.stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("Got non utf8 data from output_bytes"),
    };

    assert_eq!(output_str, "hello!!\ndefault val\n");

    Ok(())
}
