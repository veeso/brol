extern crate keyring;
extern crate whoami;

use std::io::{stdin, BufRead, Stdin};

fn read_line(input: &Stdin) -> String {
    input.lock().lines().next().expect("Nothing to read").expect("Could not read from stdin")
}

fn main() {
    // Open stdin
    let input: Stdin = stdin();
    // Read service
    eprint!("Type service name: ");
    let service: String = read_line(&input);
    eprint!("Type secret: ");
    let secret: String = read_line(&input);
    // Get keyring
    let username: String = whoami::username();
    let storage: keyring::Keyring = keyring::Keyring::new(service.as_str(), username.as_str());
    // Get current value
    match storage.get_password() {
        Ok(s) => println!("Current secret: '{}'", s),
        Err(err) => eprintln!("Could not read secret: {}", err),
    }
    // Update secret
    if let Err(err) = storage.set_password(secret.as_str()) {
        panic!("Could not save secret to storage: {}", err);
    }
    // Retrieve secret
    let storage_secret: String = match storage.get_password() {
        Ok(s) => s,
        Err(err) => panic!("Could not read secret: {}", err),
    };
    // Check if match
    assert_eq!(storage_secret, secret);
    println!("New secret: '{}'", secret);
}
