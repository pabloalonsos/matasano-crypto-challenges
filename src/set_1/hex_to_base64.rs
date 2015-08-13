extern crate rustc_serialize as serialize;


use self::serialize::base64::{STANDARD, ToBase64};
use self::serialize::hex::FromHex;

use super::super::utils::crypto_data::CryptoData;

pub fn hex_to_base64(hex_input: &str) -> String {
    CryptoData::new_from_str(hex_input)
        .from_hex()
        .to_base64()
        .to_string()
}
