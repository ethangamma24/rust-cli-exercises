use clap::Parser;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(required = true)]
    json_input: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Chair {
    Name: String,
    Price: String,
    Color: String,
    Quantity: u16,
}

fn main() {
    let args = Args::parse();

    let file_path = match args.json_input {
        Some(val) => val,
        None => panic!("JSON file required as input"),
    };

    let data: Vec<Chair> = {
        let file_content = fs::read_to_string(file_path).expect("Unable to read chairs.json");
        serde_json::from_str(&file_content).expect("Error serializing json")
    };
    let iterator = data.iter();
    let mut table = vec![];

    for chair in iterator {
        table.push(vec![
            chair.Name.clone().cell(),
            chair.Price.clone().cell(),
            chair.Color.clone().cell(),
            chair.Quantity.cell().justify(Justify::Right),
        ]);
    }

    let table = table
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Price".cell().bold(true),
            "Color".cell().bold(true),
            "Quantity".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table);
}
