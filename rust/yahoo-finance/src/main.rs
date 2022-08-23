use std::env;
use std::process::exit;
use yahoo_finance::{history, Interval};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <asset>", args.get(0).unwrap());
        exit(255);
    }

    let data = history::retrieve_interval(&args[1], Interval::_10y).unwrap();

    // print the date and closing price for each day we have data
    for bar in &data {
        println!(
            "On {} {} closed at ${:.2}",
            bar.timestamp.format("%b %e %Y"),
            args[1],
            bar.close
        )
    }
}
