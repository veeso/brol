extern crate lapin;

use futures::executor;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    Result,
};

const USAGE: &str = "rabbit-pub <queue> <payload>";

fn main() -> Result<()> {
    // Get opts
    let args: Vec<String> = std::env::args().collect();
    let queue_name: &String = args.get(1).expect(USAGE);
    let payload: &String = args.get(2).expect(USAGE);
    // Init
    let connection: Connection = executor::block_on(Connection::connect(
        "amqp://localhost",
        ConnectionProperties::default(),
    ))?;
    let channel: Channel = executor::block_on(connection.create_channel())?;
    let _ = executor::block_on(channel.queue_declare(
        queue_name,
        QueueDeclareOptions::default(),
        FieldTable::default(),
    ))?;
    executor::block_on(channel.basic_publish(
        "",
        "test",
        BasicPublishOptions::default(),
        payload.as_bytes().to_vec(),
        BasicProperties::default(),
    ))
    .map_err(|x| x)
    .map(|_| ())
}
