use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use super::super::utils::crypto_data::CryptoData;

pub fn detect_aes_ecb(data_file: &str) -> CryptoData {

    let data_path = Path::new(data_file);

    let mut input_file = match File::open(&data_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {:?}: {}", data_file, Error::description(&reason))
    };

    let mut file_contents: String = String::new();
    let _ = input_file.read_to_string(&mut file_contents);

    let contents: Vec<&str> = file_contents.split("\n").collect();

    let ( cypher_text, _ ) = contents.iter().fold(("", 0), | acc, &cypher | {

        let (_, current_score) = acc;
        let mut cypher_map: HashMap<&[u8], u8> = HashMap::new();

        for chunk in cypher.as_bytes().chunks(16) {
            if !cypher_map.contains_key(chunk) {
                cypher_map.insert(chunk, 0);
            } else {
                match cypher_map.get_mut(chunk) {
                    Some(count) => { *count += 1; },
                    None => {}
                }

                // Option 2
                //let current_val = cypher_map.remove(chunk).unwrap();
                //cypher_map.insert(chunk, current_val + 1);

                // Option 3
                // if let Some(count) = cypher_map.get_mut(chunk) {
                //     *count += 1;
                // }
            }
        }

        let score = cypher_map.values().fold(0, |acc, &val| acc + val );

        if score > current_score {
            (cypher, score)
        } else {
            acc
        }

    });

    CryptoData::new_from_str(cypher_text)

}
