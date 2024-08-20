use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Chair {
    Name: String,
    Price: String,
    Color: String,
    Quantity: u16,
}

fn main() {
    let data: Vec<Chair> = {
        let file_content = fs::read_to_string(
            "/home/thetincan/Git/Learning/rust-cli-exercises/05_table_of_chairs/src/chairs.json",
        )
        .expect("Unable to read chairs.json");
        serde_json::from_str(&file_content).expect("Error serializing json")
    };
    let iterator = data.iter();

    for chair in iterator {
        println!("{:?}", chair);
    }

    println!(
        "{:?}",
        serde_json::to_string_pretty(&data).expect("Error deserializing json")
    );
}
