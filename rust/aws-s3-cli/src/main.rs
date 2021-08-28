/**
 *
 *
 *           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                   Version 2, December 2004
 *
 *  Copyright (C) 2021 Christian Visintin
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
extern crate rpassword;
extern crate s3;

// -- mod
mod bucket;
mod command;

// -- locals
use bucket::S3Bucket;
use command::Command;

// -- ext
use std::env;
use std::io;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some("-h") = args.get(1).map(|x| x.as_str()) {
        usage();
        exit(255);
    }
    if args.len() < 3 {
        usage();
        exit(255);
    }
    let bucket_name: &String = args.get(1).unwrap();
    let region: &String = args.get(2).unwrap();
    let profile: Option<String> = args.get(3).map(|x| x.to_string());
    let mut bucket: S3Bucket =
        match S3Bucket::connect(bucket_name.as_str(), region.as_str(), profile.as_deref()) {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
    loop {
        match input() {
            Command::ChangeDir(dir) => match bucket.change_dir(dir.as_str()) {
                Ok(_) => println!("CD OK!"),
                Err(e) => println!("CD ERR: {}", e),
            },
            Command::Get(src, dest) => match bucket.get(src.as_str(), dest.as_path()) {
                Ok(_) => println!("GET OK!"),
                Err(e) => println!("GET ERR: {}", e),
            },
            Command::List(dir) => {
                bucket
                    .list(dir.as_deref())
                    .ok()
                    .unwrap()
                    .iter()
                    .for_each(|x| println!("{:?}", x));
            }
            Command::Mkdir(dir) => match bucket.mkdir(dir.as_str()) {
                Ok(_) => println!("MKDIR OK!"),
                Err(e) => println!("MKDIR ERR: {}", e),
            },
            Command::Put(src, dest) => match bucket.put(src.as_path(), dest.as_str()) {
                Ok(_) => println!("PUT OK!"),
                Err(err) => println!("PUT ERR: {}", err),
            },
            Command::Pwd => println!("PWD: {}", bucket.pwd().ok().unwrap().display()),
            Command::Remove(p) => match bucket.remove(p.as_str()) {
                Ok(_) => println!("RM OK!"),
                Err(e) => println!("RM ERR: {}", e),
            },
            Command::Stat(p) => match bucket.stat(p.as_ref()) {
                Ok(f) => println!("STAT OK: {:?}", f),
                Err(e) => println!("STAT ERR: {}", e),
            },
            Command::Quit => break,
            Command::Help => help(),
            _ => println!("Asp un sec"),
        }
    }
}

fn usage() {
    println!("Usage: aws-s3-cli <bucket> <region> [profile]");
}

fn input() -> Command {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");
        // Try to create command
        if let Ok(cmd) = Command::from_str(input.as_str()) {
            return cmd;
        }
        println!("Unknown command");
    }
}

pub fn help() {
    println!("CD <dir>                            Change working directory");
    println!("GET <file> <dest>                   Download `file` to `dest`");
    println!("HELP                                Print this help");
    println!("LIST [dir]                          List files in directory");
    println!("MKDIR <dir>                         Make directory");
    println!("PUT <file> <dest>                   Upload local file `file` to `dest`");
    println!("PWD                                 Print working directory");
    println!("QUIT                                Quit suppaftp");
    println!("RM <file>                           Remove file");
    println!("STAT <file>                         Stat `file`");
    println!();
}
