use core::panic;

use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "Calculates the nth number of the Fibonnaci Sequence. Largest number acceptable is 255"
)]
struct Args {
    #[arg(required = true)]
    seq_number: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let seq_number = match args.seq_number {
        Some(val) => val,
        None => panic!("You must enter a number"),
    };

    let fib_number = calculate_fib(String::from("0"), String::from("0"), 0, seq_number);

    println!("The number at position {seq_number:?} in the Fibonnaci Sequence is: {fib_number:?}");

    ()
}

fn calculate_fib(prev: String, curr: String, counter: u8, max: u8) -> String {
    if counter.eq(&max) {
        curr
    } else if prev.eq(&String::from("0")) && curr.eq(&String::from("0")) {
        calculate_fib(String::from("0"), String::from("1"), 1, max)
    } else {
        let prev_chars: Vec<_> = prev.chars().collect();
        let curr_chars: Vec<_> = curr.chars().collect();
    }
}
