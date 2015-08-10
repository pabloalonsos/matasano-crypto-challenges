extern crate rustc_serialize as serialize;

use std::fmt;
use self::serialize::hex::FromHex;

#[derive(Debug)]
pub struct Decrypted {
    pub key: char,
    pub score: u8,
    pub decrypted: String
}
impl fmt::Display for Decrypted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key: {}, Score: {}, Text: {}", self.key, self.score, self.decrypted.replace("\n",""))
    }
}

pub fn decrypt_xor_char(hex_input: &str) -> Decrypted {

    let hex_string = hex_input.from_hex().unwrap();
    let common_chars = "e taonrishdlfcmugypbvkjxqz".to_string().into_bytes();

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
                        score: score as u8,
                        decrypted: result
                    }
                );
            },
            Err(_) => ()
        }
    }

    result_vector.sort_by(| a, b | a.score.cmp(&b.score));

    let result: Decrypted = match result_vector.last() {
        Some(value) => Decrypted {key: value.key, score: value.score, decrypted: value.decrypted.clone()},
        None => Decrypted {key: 'A', score: 0, decrypted: "".to_string()}
    };

    Decrypted {
        key: result.key,
        score: result.score,
        decrypted: result.decrypted.clone()
    }

}
