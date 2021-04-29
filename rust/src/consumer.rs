extern crate lapin;

use futures::executor;
use futures::StreamExt;
use lapin::{
    message::Delivery, options::*, types::FieldTable, Channel, Connection, ConnectionProperties,
    Consumer, Result,
};

const USAGE: &str = "rabbit-sub <queue>";

fn main() -> Result<()> {
    // Get opts
    let args: Vec<String> = std::env::args().collect();
    let queue_name: &String = args.get(1).expect(USAGE);
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
    let mut consumer: Consumer = executor::block_on(channel.basic_consume(
        "test",
        "omar",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    ))?;
    loop {
        let (channel, delivery): (Channel, Delivery) =
            executor::block_on(consumer.next()).expect("error in consumer")?;
        println!(
            "Message '{}': \"{}\"",
            delivery.routing_key,
            std::str::from_utf8(delivery.data.as_ref()).ok().unwrap()
        );
        executor::block_on(channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()))?;
    }
}
