extern crate rustc_serialize as serialize;

use self::serialize::hex::ToHex;

pub fn repeating_key_xor(input_str: &str, key: &str) -> String {

    let input_string: String = input_str.to_string();

    let mut result: Vec<u8> = Vec::new();
    let key_bytes = key.as_bytes();
    for (i, val) in input_string.chars().enumerate() {
        match i % key.len() {
            pos => {
                result.push(val as u8 ^ key_bytes[pos] as u8)
            }
        }
    }

    result.to_hex()

}
