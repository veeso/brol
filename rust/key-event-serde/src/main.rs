use serde::{Deserialize, Serialize};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};

use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{Read, Write};

/// ## SerializerError
///
/// Contains the error for serializer/deserializer
#[derive(Debug)]
pub struct SerializerError {
    kind: SerializerErrorKind,
    msg: String,
}

/// ## SerializerErrorKind
///
/// Describes the kind of error for the serializer/deserializer
#[derive(Debug)]
pub enum SerializerErrorKind {
    Io,
    Syntax,
    Serialization,
}

impl SerializerError {
    /// ### new
    ///
    /// Instantiates a new `SerializerError` with description message
    pub fn new(kind: SerializerErrorKind, msg: String) -> SerializerError {
        SerializerError { kind, msg }
    }
}

/// ### deserialize
///
/// Read data from readable and deserialize its content as TOML
pub fn deserialize<R, S>(mut readable: R) -> Result<S, SerializerError>
where
    R: Read,
    S: DeserializeOwned + Sized + std::fmt::Debug,
{
    // Read file content
    let mut data: String = String::new();
    if let Err(err) = readable.read_to_string(&mut data) {
        return Err(SerializerError::new(
            SerializerErrorKind::Io,
            err.to_string(),
        ));
    }
    // Deserialize
    match toml::de::from_str(data.as_str()) {
        Ok(deserialized) => Ok(deserialized),
        Err(err) => Err(SerializerError::new(
            SerializerErrorKind::Syntax,
            err.to_string(),
        )),
    }
}

pub fn serialize<S, W>(serializable: &S, mut writable: W) -> Result<(), SerializerError>
where
    S: Serialize + Sized,
    W: Write,
{
    // Serialize content
    let data: String = match toml::ser::to_string(serializable) {
        Ok(dt) => dt,
        Err(err) => {
            return Err(SerializerError::new(
                SerializerErrorKind::Serialization,
                err.to_string(),
            ))
        }
    };
    // Write file
    match writable.write_all(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(SerializerError::new(
            SerializerErrorKind::Io,
            err.to_string(),
        )),
    }
}

#[derive(Deserialize, Serialize)]
pub struct KeyBindings {
    pub quit: KeyEvent,
    pub open: KeyEvent,
}

impl KeyBindings {
    pub fn new(quit: KeyEvent, open: KeyEvent) -> Self {
        Self { quit, open }
    }
}

fn main() {
    let keys = KeyBindings::new(
        KeyEvent::from(Key::Esc),
        KeyEvent::new(Key::Char('o'), KeyModifiers::CONTROL),
    );
    let mut config = File::create("keys.toml").expect("Failed to open file 'keys.toml'");
    serialize(&keys, &mut config).expect("Failed to serialize keys");
}
