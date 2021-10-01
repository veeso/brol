extern crate lapin;

use futures::executor;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use std::io;
use std::io::Write;
use std::process::exit;

const USAGE: &str = "rabbit-pub <queue>";

fn main() {
    // Get opts
    let args: Vec<String> = std::env::args().collect();
    let queue_name: &String = args.get(1).expect(USAGE);
    // Init
    let connection = match executor::block_on(Connection::connect(
        "amqp://localhost",
        ConnectionProperties::default(),
    )) {
        Err(err) => {
            eprintln!("Connection error: {}", err);
            exit(1);
        }
        Ok(c) => c,
    };
    loop {
        let payload = input();
        let channel: Channel = executor::block_on(connection.create_channel())
            .ok()
            .unwrap();
        if let Err(err) = executor::block_on(channel.queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )) {
            eprintln!("Could not create queue: {}", err);
        }
        if let Err(err) = executor::block_on(channel.basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            payload.as_bytes().to_vec(),
            BasicProperties::default(),
        ))
        .map_err(|x| x)
        {
            eprintln!("Publish error: {}", err);
        }
    }
}

fn input() -> String {
    print!(">> ");
    let _ = io::stdout().flush();
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    input
}
