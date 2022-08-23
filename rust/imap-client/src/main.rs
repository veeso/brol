#[macro_use]
extern crate serde;

use dotenv::dotenv;
use futures::TryStreamExt;
use mail_parser::*;

#[derive(Debug, Deserialize, Serialize)]
/// Application config
pub struct Config {
    imap_server: String,
    imap_port: u16,
    email_address: String,
    email_password: String,
}

impl Config {
    /// Try to create config from env
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Config::try_from_env()?;
    // setup tls
    let tls = async_native_tls::TlsConnector::new();
    // connect
    let client = async_imap::connect(
        (config.imap_server.as_str(), config.imap_port),
        &config.imap_server,
        tls,
    )
    .await
    .map_err(|e| anyhow::anyhow!("failed to connect to IMAP server: {}", e))?;
    // login
    let mut imap_session = client
        .login(&config.email_address, &config.email_password)
        .await
        .map_err(|(e, _)| anyhow::anyhow!("login failed: {}", e))?;
    // fetch inbox
    imap_session.select("INBOX").await?;
    let messages_stream = imap_session.fetch("1", "RFC822").await?;
    let messages: Vec<_> = messages_stream.try_collect().await?;
    let message = if let Some(m) = messages.first() {
        m
    } else {
        return Ok(());
    };

    // extract the message's body
    let body = match message.body() {
        Some(b) => b,
        None => anyhow::bail!("message body is empty"),
    };
    // parse message
    let message = match Message::parse(body) {
        Some(msg) => msg,
        None => anyhow::bail!("failed to parse message"),
    };

    if let HeaderValue::Address(addr) = message.get_from() {
        println!("FROM: {}", addr.name.as_ref().unwrap());
    }
    println!("DATE: {}", message.get_date().unwrap());
    println!("SUBJECT: {}", message.get_subject().unwrap());
    println!("BODY: {}", message.get_text_body(0).unwrap());

    // be nice to the server and log out
    imap_session.logout().await?;

    Ok(())
}
