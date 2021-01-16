// Crates
extern crate getopts;
extern crate keyring;
extern crate whoami;

use getopts::Options;
use keyring::Keyring;
use std::env;

/// ## CliAction
///
/// Defines what to do in client
enum CliAction {
    Del,
    Get,
    Set(String), // Value
}

/// ### print_usage
///
/// Print usage

fn print_usage(opts: Options) {
    let brief = String::from("Usage: keyring-client [options]... [service]");
    print!("{}", opts.usage(&brief));
    println!("\nPlease, report issues to <https://github.com/veeso/brol>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut action: Option<CliAction> = None;
    // Get options
    let mut opts = Options::new();
    opts.optflag("d", "del", "delete key from storage");
    opts.optflag("g", "get", "get key from storage");
    opts.optopt("s", "set", "set key into storage", "<key_value>");
    opts.optflag("h", "help", "Print this menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            std::process::exit(255);
        }
    };
    // Help
    if matches.opt_present("h") {
        print_usage(opts);
        std::process::exit(255);
    }
    // Match del
    if matches.opt_present("d") {
        action = Some(CliAction::Del);
    }
    // Match get
    if matches.opt_present("g") {
        action = Some(CliAction::Get);
    }
    // Match set
    if let Some(val) = matches.opt_str("s") {
        action = Some(CliAction::Set(val));
    }
    // Get service
    let extra_args: Vec<String> = matches.free;
    let service: String = match extra_args.get(0) {
        Some(s) => s.to_string(),
        None => {
            print_usage(opts);
            std::process::exit(255);
        }
    };
    // Run client
    let rc: i32 = match action {
        None => {
            print_usage(opts);
            255
        }
        Some(action) => {
            // Create keyring
            let username: String = whoami::username();
            let storage: Keyring = Keyring::new(service.as_str(), username.as_str());
            match action {
                CliAction::Del => match storage.delete_password() {
                    Ok(_) => {
                        println!("Password deleted!");
                        0
                    }
                    Err(e) => {
                        eprintln!("Could not delete password: {}", e);
                        1
                    }
                },
                CliAction::Get => match storage.get_password() {
                    Ok(val) => {
                        println!("{}", val);
                        0
                    }
                    Err(e) => {
                        eprintln!("Could not get password: {}", e);
                        1
                    }
                },
                CliAction::Set(val) => match storage.set_password(val.as_str()) {
                    Ok(_) => {
                        println!("Password set!");
                        0
                    }
                    Err(e) => {
                        eprintln!("Could not set password: {}", e);
                        1
                    }
                },
            }
        }
    };
    std::process::exit(rc);
}
