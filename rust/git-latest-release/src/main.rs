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
extern crate ureq;

use serde::Deserialize;

#[derive(Deserialize)]
struct TagInfo {
    tag_name: String,
}

use std::env;
use std::process::exit;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    // Check args len
    if args.len() < 3 {
        eprintln!("Usage: {} <author> <reponame>", args.get(0).unwrap());
        exit(255);
    }

    let author: String = args.get(1).unwrap().to_string();
    let repository: String = args.get(2).unwrap().to_string();
    // make url
    let url: String = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        author, repository
    );
    println!("Sending request to \"{}\"", url);
    // Send request
    let body = match ureq::get(url.as_str()).call() {
        Ok(response) => response,
        Err(err) => panic!("Request failed API: {}", err),
    };
    let body: TagInfo = match body.into_json() {
        Ok(b) => b,
        Err(err) => panic!("Could not parse response: {}", err),
    };
    // get latest tag
    println!("Latest {} version: {}", repository, body.tag_name);
}
