extern crate notify;

use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: fs-notify <path>...");
        exit(255);
    };

    let paths = &args.as_slice()[1..];

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    for p in paths.iter() {
        watcher.watch(p.as_str(), RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
