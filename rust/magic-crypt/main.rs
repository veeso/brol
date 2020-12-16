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

#[macro_use] extern crate magic_crypt;
extern crate rand;

use magic_crypt::MagicCryptTrait;
use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::process::exit;

fn gen_key() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(256)
        .collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check args len
    if args.len() < 2 {
        eprintln!("Usage: {} <secret>", args.get(0).unwrap());
        exit(255);
    }
    let secret: String = args.get(1).unwrap().clone();
    let key: String = gen_key();
    println!("Key is \"{}\"", key);
    // Prepare crypter, AES-128
    let crypter = new_magic_crypt!(key, 256);
    // Encrypt and convert to base64
    let encrypted: String = crypter.encrypt_str_to_base64(secret.clone());
    println!("Encrypted secret: \"{}\"", encrypted);
    // Decrypt
    let decrypted: String = crypter.decrypt_base64_to_string(encrypted).ok().unwrap();
    println!("Decrypted secret: \"{}\"", decrypted);
    assert_eq!(secret, decrypted);
}
