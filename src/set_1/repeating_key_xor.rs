extern crate rustc_serialize as serialize;

use self::serialize::hex::ToHex;

pub fn repeating_key_xor(input_str: &str) -> String {

    let input_string: String = input_str.to_string();

    let mut result: Vec<u8> = Vec::new();
    for (i, val) in input_string.chars().enumerate() {
        match i % 3 {
            0 => result.push(val as u8 ^ b'I'),
            1 => result.push(val as u8 ^ b'C'),
            2 => result.push(val as u8 ^ b'E'),
            _ => panic!("Something went wrong")
        }
    }

    result.to_hex()
}
