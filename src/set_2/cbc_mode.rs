use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::super::utils::crypto_data::CryptoData;

pub fn cbc_mode(data_file: &str, input_key: &str) -> CryptoData {

    let data_path = Path::new(data_file);

    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {:?}: {}", data_file, Error::description(&reason))
    };

    let mut contents: String = String::new();
    let _ = input_file.read_to_string(&mut contents);

    let crypto_contents: CryptoData = CryptoData::new_from_str(&contents);

    let iv: CryptoData = CryptoData::new_from_vec(vec![0; 16]); // 16 zeroes as Initialization Vector

    let decrypted_content = crypto_contents.from_base64().decrypt_aes_128_cbc(input_key.as_bytes().to_vec(), iv.clone());

    let _ = decrypted_content.encrypt_aes_128_cbc(input_key.as_bytes().to_vec(), iv.clone()); // left here for reference for encryption

    decrypted_content

}
