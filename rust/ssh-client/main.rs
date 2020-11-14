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
// Dependencies
extern crate rpassword;
extern crate ssh2;

// Includes
use ssh2::Session;
use std::env;
use std::io;
use std::io::*;
use std::net::TcpStream;
use std::process::exit;
// Threading
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check args len
    if args.len() < 2 {
        eprintln!("Usage: {} <address> [port]", args.get(0).unwrap());
        exit(255);
    }
    let address: String = args.get(1).unwrap().clone();
    let port: u16 = match args.get(2) {
        Some(p) => p.parse::<u16>().unwrap(),
        None => 22,
    };
    // Create session
    println!("Connecting to {}:{}", address, port);
    let tcp = TcpStream::connect(format!("{}:{}", address, port)).unwrap();
    // Create session
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    println!("Connection established");
    // Ask for username
    println!("Type username");
    let mut username = String::new();
    let _ = io::stdin().read_line(&mut username);
    let password: String = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    // Trim
    trim_newline(&mut username);
    println!("Authenticating with '{}'", username);
    // Try to authenticate
    session
        .userauth_password(username.as_str(), password.as_str())
        .unwrap();
    if !session.authenticated() {
        eprintln!("Authentication failed...");
        exit(1);
    }
    // Print banner
    if let Some(banner) = session.banner() {
        println!("{}", banner);
    }
    // Request pty
    let mut mode = ssh2::PtyModes::new();
    mode.set_character(ssh2::PtyModeOpcode::VINTR, Some(3 as char));
    let mut channel = session.channel_session().unwrap();
    if let Err(err) = channel.request_pty("vt100", Some(mode), None) {
        eprintln!("Could not get pty: {}", err);
        exit(1);
    }
    if let Err(err) = channel.shell() {
        eprintln!("Failed to start shell: {}", err);
        exit(1);
    }
    // Set blocking to false
    session.set_blocking(false);
    // Prepare stdin listener
    let stdin_channel = spawn_stdin_channel();
    // Until ssh session has terminated
    while !channel.eof() {
        match stdin_channel.try_recv() {
            Ok(user_input) => {
                // Write
                if let Err(err) = channel.write(user_input.as_str().as_bytes()) {
                    eprintln!("Write failed: {}", err);
                    exit(1);
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        // Read output
        let mut output: String = String::new();
        let mut buffer: [u8; 8192] = [0; 8192];
        match channel.read(&mut buffer) {
            Ok(bytes_read) => {
                output.push_str(std::str::from_utf8(&buffer[0..bytes_read]).unwrap());
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::WouldBlock {
                    continue; // Ignore
                } else {
                    eprintln!("Could not read output: {}", err);
                    exit(1);
                }
            }
        }
        let _ = channel.wait_close();
        if output.len() > 0 {
            print!("{}", output);
            // Flush
            io::stdout().flush().unwrap();
        }
        thread::sleep(Duration::from_millis(10));
    }
    // Close session
    let _ = session.disconnect(None, "mandi", None);
    exit(0);
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
