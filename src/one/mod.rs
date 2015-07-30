extern crate rustc_serialize as serialize;

use std::fmt;
use self::serialize::base64::{STANDARD, ToBase64};
use self::serialize::hex::{ToHex, FromHex};

fn convert_hex_to_base64(hex_input: &str) {
    let base_64 = hex_input.from_hex().unwrap().to_base64(STANDARD);
    println!("Result >>> {:?}", base_64);
}

fn fixed_xor(input_1: &str, input_2: &str) {
    let input_1_vec = input_1.from_hex().unwrap();
    let input_2_vec = input_2.from_hex().unwrap();
    let input_1_iter = input_1_vec.iter();
    let input_2_iter = input_2_vec.iter();

    let zipped_input = input_1_iter.zip(input_2_iter);

    let result: Vec<u8> = zipped_input
        .map(| (&a, &b) | a^b )
        .collect();

    println!("Result >>> {:?}", result.to_hex());
}

#[derive(Debug)]
struct Decrypted {
    key: char,
    score: u8,
    decrypted: String
}
impl fmt::Display for Decrypted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key: {}, Score: {}, Text: {}", self.key, self.score, self.decrypted.replace("\n",""))
    }
}

fn decrypt_xor_char(hex_input: &str) {

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

    println!("Result >>> {}", result_vector.last().unwrap());

}

pub fn set_1() {
    println!("");
    println!("Set 1:");
    println!("");

    println!("Exercise 1: Convert HEX to Base64 (http://cryptopals.com/sets/1/challenges/1/)");
    convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("");

    println!("Exercise 2: Fixed XOR (http://cryptopals.com/sets/1/challenges/2/)");
    let input_1 = "1c0111001f010100061a024b53535009181c";
    let input_2 = "686974207468652062756c6c277320657965";
    fixed_xor(input_1, input_2);
    println!("");

    println!("Exercise 3: Single-byte XOR cipher (http://cryptopals.com/sets/1/challenges/3/)");
    decrypt_xor_char("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("");

}
