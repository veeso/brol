extern crate notify;

use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: fs-notify <path>");
            exit(255);
        }
    };

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();

    watcher
        .watch(path.as_str(), RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
