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

extern crate num;
#[macro_use]
extern crate num_derive;

use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
pub enum ErrorCode {
    ItsRainingEggs = 0xA0,
    ShopIsClosed = 0x1F,
    CartIsEmpty = 0x10,
    BadCreditCard = 0x04,
}

impl TryFrom<u8> for ErrorCode {
    type Error = &'static str;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match num::FromPrimitive::from_u8(code) {
            Some(err) => Ok(err),
            None => Err("Unknown error code"),
        }
    }
}

fn main() {
    assert_eq!(
        ErrorCode::try_from(160).ok().unwrap(),
        ErrorCode::ItsRainingEggs
    );
    assert_eq!(
        ErrorCode::try_from(31).ok().unwrap(),
        ErrorCode::ShopIsClosed
    );
    assert_eq!(
        ErrorCode::try_from(16).ok().unwrap(),
        ErrorCode::CartIsEmpty
    );
    assert_eq!(
        ErrorCode::try_from(4).ok().unwrap(),
        ErrorCode::BadCreditCard
    );
    assert!(ErrorCode::try_from(1).is_err());
}
