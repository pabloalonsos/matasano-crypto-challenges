extern crate rustc_serialize as serialize;

use self::serialize::base64::FromBase64;
use self::serialize::hex::{ToHex, FromHex};

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ops::BitXor;

use super::decrypt_xor_char::decrypt_xor_char as decrypt_xor_char;
use super::repeating_key_xor::repeating_key_xor as repeating_key_xor;
use super::decrypt_xor_char::Decrypted;

fn calculate_hamming_distance(input_1: &str, input_2: &str) -> u16 {
    let input_1_vec: Vec<u8> = input_1.to_string().into_bytes();
    let input_2_vec: Vec<u8> = input_2.to_string().into_bytes();
    input_1_vec
        .iter()
        .zip(input_2_vec.iter())
        .fold(0, | acc, (&value_1, &value_2) | acc + (value_1.bitxor(value_2).count_ones() as u16))
}

fn get_xor_keysize(data_line: &str) -> u16 {
    let (_, keysize) = (2..40)
        .fold((1000.0, 0), |acc, keysize| {
            let (acc_dist, _) = acc;

            let mut sum = 0;
            let count = data_line.len() / keysize - 1;

            for i in (0..count) {
                let distance = calculate_hamming_distance(&data_line[(i * keysize)..((i+1)*keysize)], &data_line[((i+1)*keysize)..((i+2)*keysize)]);
                sum += distance;
            }
            let avg_distance: f32 = sum as f32 / count as f32 / keysize as f32;

            if avg_distance < acc_dist {
                (avg_distance, keysize as u16)
            } else { acc }
        });
    keysize
}

pub fn break_repeating_key_xor(data_file: &str) -> String {

    let data_path = Path::new(data_file);

    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {:?}: {}", data_file, Error::description(&reason))
    };

    let mut base64_contents: String = String::new();
    let _ = input_file.read_to_string(&mut base64_contents);

    let contents: Vec<u8> = base64_contents.from_base64().unwrap();

    let contents_string: String = String::from_utf8(contents.clone()).unwrap();
    let keysize: u16 = get_xor_keysize(&contents_string);

    let mut transposed_result: Vec<Vec<u8>> = Vec::new();
    for position in (0..keysize) {
        let mut transpose: Vec<u8> = Vec::new();
        for transpose_pos in (contents_string.as_bytes().chunks(keysize as usize)) {
            if transpose_pos.len() > (position as usize) {
                transpose.push(transpose_pos[position as usize]);
            }
        }
        transposed_result.push(transpose);
    }

    let mut key: String = String::new();
    for transposed_vec in transposed_result {
        let transposed_item: String = (&transposed_vec).to_hex();
        let decrypted: Decrypted = decrypt_xor_char(&transposed_item);
        key = key + &(decrypted.key.to_string());
    }

    let final_result: Vec<u8> = repeating_key_xor(&contents_string, &key).from_hex().unwrap();

    String::from_utf8(final_result).unwrap()

}
