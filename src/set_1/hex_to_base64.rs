extern crate rustc_serialize as serialize;

use self::serialize::base64::{STANDARD, ToBase64};
use self::serialize::hex::FromHex;

pub fn hex_to_base64(hex_input: &str) -> String {
    hex_input.from_hex().unwrap().to_base64(STANDARD)
}
