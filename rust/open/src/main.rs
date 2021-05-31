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

extern crate open;

use std::env;
use std::io::Result as IoResult;
use std::path::PathBuf;
use std::process::ExitStatus;
use std::thread::JoinHandle;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <file>", args[0]);
    }
    let file: PathBuf = PathBuf::from(args[1].as_str());
    let join: JoinHandle<IoResult<ExitStatus>> = open::that_in_background(file.as_path());
    println!("Opened {}", file.display());
    match join.join() {
        Ok(rc) => println!("Process exited with {:?}", rc),
        Err(err) => eprintln!("Failed to open file: {:?}", err),
    }
}
