extern crate rustc_serialize as serialize;

use std::fmt;
use self::serialize::hex::FromHex;

#[derive(Debug)]
pub struct Decrypted {
    key: char,
    score: u8,
    decrypted: String
}
impl fmt::Display for Decrypted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key: {}, Score: {}, Text: {}", self.key, self.score, self.decrypted.replace("\n",""))
    }
}

pub fn decrypt_xor_char(hex_input: &str) -> String {

    let hex_string = hex_input.from_hex().unwrap();
    let common_chars = "etaonrishdlfcmugypwbvkjxqz".to_string().into_bytes();

    let mut result_vector: Vec<Decrypted> = Vec::new();
    for i in 0..255 {
        let mut score = 0;
        let xor_key = i as u8;
        let decrypted: Vec<u8> = hex_string
            .iter()
            .map(|&a| {
                let decrypted_char = a^xor_key;
                for common_char in &common_chars[..] {
                    if *common_char == decrypted_char {
                        score = score + 1;
                    }
                }
                decrypted_char
            })
            .collect();

        match String::from_utf8(decrypted){
            Ok(result) => {
                result_vector.push(
                    Decrypted {
                        key: xor_key as char,
                        score: score,
                        decrypted: result
                    }
                );
            },
            Err(err) => {}
        }
    }

    result_vector.sort_by(| a, b | a.score.cmp(&b.score));

    result_vector.last().unwrap().to_string()

}
