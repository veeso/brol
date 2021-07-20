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

extern crate hex;
extern crate sha1;

use sha1::{Digest, Sha1};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check args len
    if args.len() < 2 {
        eprintln!("Usage: {} <s>", args.get(0).unwrap());
        exit(255);
    }
    let target: String = args.get(1).unwrap().clone();
    let mut hasher: Sha1 = Sha1::default();
    hasher.update(target.as_bytes());
    let result = hasher.finalize();
    let result: String = hex::encode(result);
    println!("{}", result);
}
