extern crate rustc_serialize as serialize;

use std::fmt;
use std::ops::BitXor;
use self::serialize::hex::{ToHex, FromHex};
use self::serialize::base64::{STANDARD, ToBase64, FromBase64};

fn get_char_score(input_char: char) -> i32 {
    match input_char {
        'e' => 27,
        ' ' => 26,
        't' => 25,
        'a' => 24,
        'o' => 23,
        'n' => 22,
        'r' => 21,
        'i' => 20,
        's' => 19,
        'h' => 18,
        'd' => 17,
        'l' => 16,
        'f' => 15,
        'c' => 14,
        'm' => 13,
        'u' => 12,
        'g' => 11,
        'y' => 10,
        'p' => 9,
        'b' => 8,
        'v' => 6,
        'k' => 5,
        'j' => 4,
        'x' => 3,
        'q' => 2,
        'z' => 1,
        '\x00'...'\x19' => -10,
        _ => 0
    }
}

#[derive(Debug)]
pub struct CryptoData {
    data: Vec<u8>
}

impl fmt::Display for CryptoData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.data.clone()).unwrap())
    }
}

impl CryptoData {

    pub fn new() -> CryptoData {
        CryptoData {
            data: Vec::new()
        }
    }

    pub fn new_from_vec(input: Vec<u8>) -> CryptoData {
        CryptoData {
            data: input.clone()
        }
    }

    pub fn new_from_str(input: &str) -> CryptoData {
        CryptoData {
            data: input.as_bytes().to_vec()
        }
    }

    pub fn len(&self) -> usize {
        self.to_vec().len() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.to_vec().is_empty()
    }

    pub fn clone(&self) -> CryptoData {
        CryptoData::new_from_vec(
            self
                .to_vec()
                .clone()
        )
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn to_string(&self) -> String {
        match String::from_utf8(self.data.clone()) {
            Ok(result) => result,
            Err(err) => panic!("There was an error converting {} into a string: {}", self, err)
        }
    }

    pub fn from_base64(&self) -> CryptoData {
        CryptoData::new_from_vec(
            self
                .to_vec()
                .from_base64()
                .unwrap()
        )
    }

    pub fn to_base64(&self) -> CryptoData {
        CryptoData::new_from_str(
            &self
                .to_vec()
                .to_base64(STANDARD)
        )
    }

    pub fn from_hex(&self) -> CryptoData {
        CryptoData::new_from_vec(
            self
                .to_string()
                .from_hex()
                .unwrap()
        )
    }


    pub fn to_hex(&self) -> CryptoData {
        CryptoData::new_from_str(
            &self
                .to_vec()
                .to_hex()
        )
    }

    pub fn xor(&self, key: CryptoData) -> CryptoData {
        let mut result: Vec<u8> = Vec::new();
        let key_bytes = key.to_vec();
        for (i, val) in self.to_vec().iter().enumerate() {
            match i % key.to_vec().len() {
                pos => result.push(val ^ key_bytes[pos])
            }
        }
        CryptoData::new_from_vec(result)
    }

    pub fn score(&self) -> i32 {
        self
            .to_vec()
            .iter()
            .fold(0, | score, &data_char | {
                score + get_char_score(data_char as char)
            })
    }

    pub fn decrypt_single_xor(&self) -> (i32, CryptoData, CryptoData) {

        let mut best_xored: CryptoData = CryptoData::new();
        let mut best_score: i32 = 0;
        let mut best_key: CryptoData = CryptoData::new();

        for i in 0..255 {
            let xor_key = vec![i];
            let current_key = CryptoData::new_from_vec(xor_key);
            let current_xored = self.xor(current_key.clone());
            let current_score = current_xored.score();

            if current_score > best_score {
                best_xored = current_xored;
                best_score = current_score;
                best_key = current_key;
            }
        }

        (best_score, best_key, best_xored)

    }

    pub fn hamming_distance(&self, input: CryptoData) -> u16 {
        self
            .to_vec()
            .iter()
            .zip(input.to_vec().iter())
            .fold(0, | acc, (&value_1, &value_2) | acc + (value_1.bitxor(value_2).count_ones() as u16))
    }

    pub fn xor_keysize(&self) -> u16 {
        let (_, keysize) = (2..40)
            .fold((1000.0, 0), |acc, keysize| {
                let (acc_dist, _) = acc;

                let mut sum = 0;
                let count = self.len() / keysize - 1;
                let chunk = self.to_vec();
                let mut t = chunk.chunks(keysize as usize).peekable();

                while !t.peek().is_none() {

                    let a = if !t.peek().is_none() {
                        CryptoData::new_from_vec(t.next().unwrap().to_vec())
                    } else { CryptoData::new() };

                    let b = if !t.peek().is_none() {
                        CryptoData::new_from_vec(t.next().unwrap().to_vec())
                    } else { CryptoData::new() };

                    if !a.is_empty() && !b.is_empty() {
                        sum += a.hamming_distance(b);
                    }

                }

                let avg_distance: f32 = sum as f32 / count as f32 / keysize as f32;

                if avg_distance < acc_dist {
                    (avg_distance, keysize as u16)
                } else {
                    acc
                }
            });

        keysize
    }

}
