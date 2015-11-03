use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::super::utils::crypto_data::CryptoData;

pub fn aes_ecb_mode(data_file: &str) -> CryptoData {

    let data_path = Path::new(data_file);

    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {:?}: {}", data_file, Error::description(&reason))
    };

    let mut base64_contents: String = String::new();
    let _ = input_file.read_to_string(&mut base64_contents);

    let contents: CryptoData = CryptoData::new_from_str(&base64_contents).from_base64();

    contents.aes_128_ecb_decrypt("YELLOW SUBMARINE".as_bytes().to_vec())
}
