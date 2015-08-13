use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::super::utils::crypto_data::CryptoData;

pub fn break_repeating_key_xor(data_file: &str) -> CryptoData {

    let data_path = Path::new(data_file);

    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {:?}: {}", data_file, Error::description(&reason))
    };

    let mut base64_contents: String = String::new();
    let _ = input_file.read_to_string(&mut base64_contents);

    let contents: CryptoData = CryptoData::new_from_str(&base64_contents).from_base64();

    let keysize: u16 = contents.xor_keysize();

    let mut transposed_result: Vec<Vec<u8>> = Vec::new();
    for position in (0..keysize) {
        let mut transpose: Vec<u8> = Vec::new();
        for transpose_pos in (contents.to_vec().chunks(keysize as usize)) {
            if transpose_pos.len() > (position as usize) {
                transpose.push(transpose_pos[position as usize]);
            }
        }
        transposed_result.push(transpose);
    }

    let mut key: String = String::new();
    for transposed_vec in transposed_result {
        let (_, decrypted_key, _): (i32, CryptoData, CryptoData) = CryptoData::new_from_vec(transposed_vec).decrypt_single_xor();
        key = key + &decrypted_key.to_string();
    }

    contents.xor(CryptoData::new_from_str(&key))

}
