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
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Pwd,
    ChangeDir(String),
    Copy(String, String),
    Help,
    List(Option<String>),
    Mkdir(String),
    Remove(String),
    Stat(String),
    Put(PathBuf, String),
    Get(String, PathBuf),
    Quit,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split string by space
        let mut args = s.split_ascii_whitespace();
        // Match args
        match args.next() {
            Some(cmd) => match cmd.to_ascii_uppercase().as_str() {
                "PWD" => Ok(Self::Pwd),
                "QUIT" => Ok(Self::Quit),
                "CD" => match args.next() {
                    Some(p) => Ok(Self::ChangeDir(p.to_string())),
                    None => Err("Missing `dir` field"),
                },
                "COPY" => {
                    let src: String = match args.next() {
                        Some(s) => s.to_string(),
                        None => return Err("Missing `src` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Copy(src, d.to_string())),
                        None => Err("Missing `dest` field"),
                    }
                }
                "HELP" => Ok(Self::Help),
                "LIST" => match args.next() {
                    Some(p) => Ok(Self::List(Some(p.to_string()))),
                    None => Ok(Self::List(None)),
                },
                "MKDIR" => match args.next() {
                    Some(p) => Ok(Self::Mkdir(p.to_string())),
                    None => Err("Missing `dir` field"),
                },
                "RM" => match args.next() {
                    Some(p) => Ok(Self::Remove(p.to_string())),
                    None => Err("Missing `file` field"),
                },
                "STAT" => match args.next() {
                    Some(p) => Ok(Self::Stat(p.to_string())),
                    None => Err("Missing `dir` field"),
                },
                "PUT" => {
                    let src: PathBuf = match args.next() {
                        Some(s) => PathBuf::from(s),
                        None => return Err("Missing `src` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Put(src, d.to_string())),
                        None => Err("Missing `dest` field"),
                    }
                }
                "GET" => {
                    let src: String = match args.next() {
                        Some(s) => s.to_string(),
                        None => return Err("Missing `src` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Get(src, PathBuf::from(d))),
                        None => Err("Missing `dest` field"),
                    }
                }
                _ => Err("Unknown command"),
            },
            None => Err("Unknown command"),
        }
    }
}
