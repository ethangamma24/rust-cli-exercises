#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    println!("{}", dotenv!("MY_VAR_1"));

    match env::var_os("MY_VAR_2") {
        Some(val) => println!("{val:?}"),
        None => println!("default val"),
    }
}
