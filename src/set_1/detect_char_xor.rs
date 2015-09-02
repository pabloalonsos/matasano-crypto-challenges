use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::super::utils::crypto_data::CryptoData;

pub fn detect_char_xor (data_file: &str) -> (i32, CryptoData, CryptoData) {
    // Create path to file
    let data_path = Path::new(data_file);
    let data_path_string = data_path.display();

    // Open file in path in read-only mode
    let mut input_file = match File::open(data_file) {
        Ok(file) => file,
        Err(reason) => panic!("couldn't open {}: {}", data_path_string, Error::description(&reason))
    };

    // Read file content's into vector
    let mut input_data: String = String::new();
    let _ = match input_file.read_to_string(&mut input_data) {
        Ok(res) => res,
        Err(reason) => panic!("couldn't read file: {}", Error::description(&reason))
    };

    let file_lines: Vec<&str> = input_data.split("\n").collect();
    let (best_score, best_key, best_decrypted_text): (i32, CryptoData, CryptoData) = file_lines
        .iter()
        .fold((0, CryptoData::new(), CryptoData::new()), | acc, input_line| {
            let (current_best_score, _, _) = acc;
            let (score, key, decrypted_text): (i32, CryptoData, CryptoData) = CryptoData::new_from_str(input_line).from_hex().decrypt_single_xor();
            if score > current_best_score {
                (score, key, decrypted_text)
            } else {
                acc
            }
        });

    (best_score, best_key, best_decrypted_text)
}
