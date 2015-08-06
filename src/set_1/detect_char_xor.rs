use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::decrypt_xor_char::decrypt_xor_char as decrypt_xor_char;
use super::decrypt_xor_char::Decrypted;

pub fn detect_char_xor (data_file: &str) -> Decrypted {
    // Create path to file
    let data_path = Path::new(data_file);
    let data_path_string = data_path.display();

    // Open file in path in read-only mode
    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("couldn't open {}: {}", data_path_string, Error::description(&reason))
    };

    // Read file content's into vector
    let mut input_data: String = String::new();
    let mut input_vector: Vec<Decrypted> = Vec::new();
    match input_file.read_to_string(&mut input_data) {
        Ok(_) => {
            let file_lines: Vec<&str> = input_data.split("\n").collect();
            for input_line in file_lines {
                input_vector.push(decrypt_xor_char(&input_line));
            }
        },
        Err(reason) => panic!("couldn't read {}: {}", data_path_string, Error::description(&reason))
    };

    input_vector.sort_by(| a, b | a.score.cmp(&b.score));
    let result: Decrypted = match input_vector.last() {
        Some(value) => Decrypted {key: value.key, score: value.score, decrypted: value.decrypted.clone()},
        None => Decrypted {key: 'A', score: 0, decrypted: "".to_string()}
    };
    Decrypted {
        key: result.key,
        score: result.score,
        decrypted: result.decrypted.clone()
    }

}
