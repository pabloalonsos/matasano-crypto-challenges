extern crate rustc_serialize as serialize;

use self::serialize::hex::{ToHex, FromHex};

pub fn fixed_xor(input_1: &str, input_2: &str) -> String {
    let input_1_vec = input_1.from_hex().unwrap();
    let input_2_vec = input_2.from_hex().unwrap();
    let input_1_iter = input_1_vec.iter();
    let input_2_iter = input_2_vec.iter();

    let zipped_input = input_1_iter.zip(input_2_iter);

    let result: Vec<u8> = zipped_input
        .map(| (&a, &b) | a^b )
        .collect();

    result.to_hex()
}
