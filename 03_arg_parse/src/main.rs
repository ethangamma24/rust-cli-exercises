use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Prints out the args you pass in")]
struct Args {
    #[arg(required = true)]
    first: Option<String>,

    #[arg(required = false)]
    second: Option<String>,

    #[arg(short, long, required = false)]
    flag: Option<u16>,
}

fn main() {
    let args = Args::parse();

    match args.first {
        Some(val) => println!("{val:?}"),
        None => panic!("Expected at least one string argument"),
    }
    if let Some(val) = args.second {
        println!("{val:?}");
    }
    match args.flag {
        Some(val) => println!("{val:?}"),
        None => println!("None"),
    };
}
