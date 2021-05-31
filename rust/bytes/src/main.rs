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

extern crate bytes;

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait Encode: Sized {
    fn encode(&self) -> Bytes;
}

pub trait Decode: Sized {
    fn decode(buf: &mut dyn Buf) -> Result<Self, ()>;
}

#[derive(Debug, PartialEq)]
pub struct Negotiation {
    version: u16, // 2
    name: String, // len + 2
    key: u32,     // 4
    secure: bool, // 1
}

impl Encode for Negotiation {
    fn encode(&self) -> Bytes {
        let buf_size: usize = 9 + self.name.len();
        let mut buf: BytesMut = BytesMut::with_capacity(buf_size);
        // Encode
        buf.put_u16(self.version);
        buf.put_u16(self.name.len() as u16);
        buf.put(self.name.as_bytes());
        buf.put_u32(self.key);
        buf.put_u8(self.secure as u8);
        buf.freeze()
    }
}

impl Decode for Negotiation {
    fn decode(buf: &mut dyn Buf) -> Result<Self, ()> {
        if buf.remaining() < 9 {
            // min size
            eprintln!("Expected size {}; got {}", 13, buf.remaining());
            return Err(());
        }
        let version: u16 = buf.get_u16();
        let name_len: usize = buf.get_u16() as usize;
        if buf.remaining() < name_len + 5 {
            // Minimum size
            eprintln!("Expected size {}; got {}", name_len + 9, buf.remaining());
            return Err(());
        }
        let mut name_buf: Vec<u8> = vec![0; name_len];
        buf.copy_to_slice(name_buf.as_mut_slice());
        let name: String = match std::str::from_utf8(name_buf.as_slice()) {
            Ok(n) => n.to_string(),
            Err(_) => return Err(()),
        };
        let key: u32 = buf.get_u32();
        let secure: bool = buf.get_u8() != 0;
        Ok(Negotiation {
            version,
            name,
            key,
            secure,
        })
    }
}

fn main() {
    let msg: Negotiation = Negotiation {
        version: 0xcafe,
        name: String::from("drome-dario"),
        key: 0xcafebabe,
        secure: true,
    };
    let encoded: Vec<u8> = msg.encode().to_vec();
    assert_eq!(
        encoded,
        vec![
            0xca, 0xfe, 0x00, 0x0b, 0x64, 0x72, 0x6f, 0x6d, 0x65, 0x2d, 0x64, 0x61, 0x72, 0x69,
            0x6f, 0xca, 0xfe, 0xba, 0xbe, 0x01,
        ]
    );
    // Decode
    let mut buffer: Bytes = Bytes::from(encoded);
    let in_msg: Negotiation = Negotiation::decode(&mut buffer).ok().unwrap();
    assert_eq!(msg, in_msg);
}
