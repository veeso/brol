/**
 *
 *
 *           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                   Version 2, December 2004
 *
 *  Copyright (C) 2020 Christian Visintin
 *
 *  Everyone is permitted to copy and distribute verbatim or modified
 *  copies of this license document, and changing it is allowed as long
 *  as the name is changed.
 *
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 *   0. You just DO WHAT THE FUCK YOU WANT TO.
*/

use std::io;
use std::io::*;
use std::thread::sleep;
use std::time::Duration;

/// ### print_progress_bar
/// 
/// Print progress bar to stdout
fn print_progress_bar(it: usize, max: usize, prefix: &str) {
    let percentage: f64 = ((it as f64) * 100.0) / (max as f64);
    // Allocate bar
    let mut prog_bar: String = String::with_capacity(100);
    // For 100 times
    for i in 0..100 {
        if i <= percentage as i64 {
            prog_bar.push_str("█");
        } else {
            prog_bar.push_str(" ");
        }
    }
    // Print
    print!("\r{} [{}] {:.2}%", prefix, prog_bar, percentage);
    if it >= max {
        println!("");
    } else {
        // Flush
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let mut it: usize = 0;
    let max: usize = 4096;
    while it < max {
        print_progress_bar(it, max, "Loading...");
        it = it + 1;
        sleep(Duration::from_millis(1));
    }
}
