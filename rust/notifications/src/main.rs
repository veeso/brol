extern crate notify_rust;

use notify_rust::Notification;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: notifications <title> <body>");
        exit(255);
    }
    let title = args.get(0).to_owned().unwrap();
    let body = args.get(1).to_owned().unwrap();
    if let Err(err) = Notification::new()
        .summary(title.as_str())
        .body(body.as_str())
        .appname("termscp")
        .timeout(5000)
        .show()
    {
        eprintln!("Could not display notification: {}", err);
        exit(1);
    }
}
