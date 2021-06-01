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
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <file> [open-with]", args[0]);
    }
    let file: PathBuf = PathBuf::from(args[1].as_str());
    let open_with: Option<String> = match args.get(2) {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    /*
    println!("Open async...");
    let join: JoinHandle<IoResult<ExitStatus>> = open::that_in_background(file.as_path());
    println!("Opened {}", file.display());
    match join.join() {
        Ok(rc) => println!("Process exited with {:?}", rc),
        Err(err) => eprintln!("Failed to open file: {:?}", err),
    }
    */
    match open::that(file.as_path()) {
        Ok(rc) => println!("Process exited with {:?}", rc),
        Err(err) => eprintln!("Failed to open file: {:?}", err),
    }
    if let Some(open_with) = open_with {
        println!("Open with {}", open_with);
        match open::with(file.as_path(), open_with) {
            Ok(rc) => println!("Process exited with {:?}", rc),
            Err(err) => eprintln!("Failed to open file: {:?}", err),
        }
    }
}
