extern crate openssl;

use self::openssl::crypto::rand::rand_bytes;

use std::collections::HashMap;

use super::super::utils::crypto_data::CryptoData;

fn block_size(crypto_appended_text: &CryptoData, random_key: &Vec<u8>) -> usize {
    let first_block_size: usize = CryptoData::new_from_str("A")
        .concat(crypto_appended_text.clone())
        .aes_128_ecb_encrypt(random_key)
        .len();

    (0..64).fold(0, |acc, block_size| {
        let input_text: String = (0..block_size).map(|_| "A").collect();
        let crypto_input_block_size: usize = CryptoData::new_from_str(&input_text)
            .concat(crypto_appended_text.clone())
            .aes_128_ecb_encrypt(random_key)
            .len();

        if (acc == 0 && crypto_input_block_size > first_block_size) {
            crypto_input_block_size - first_block_size
        } else {
            acc
        }
    })
}

fn byte_ECB_oracle(custom_text: &str, crypto_input: &CryptoData, random_key: &Vec<u8>) -> CryptoData {
    CryptoData::new_from_str(custom_text)
        .concat(crypto_input.clone())
        .aes_128_ecb_encrypt(random_key)
}

pub fn byte_ECB_decryption() -> CryptoData {
    let appended_text: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let crypto_appended_text = CryptoData::new_from_str(appended_text).from_base64();
    let random_key: Vec<u8> = rand_bytes(16);

    let crypto_block_size = block_size(&crypto_appended_text, &random_key);

    println!("Crypto Block Size: {}", crypto_block_size);

    let test = crypto_appended_text
        .to_vec()
        .chunks(crypto_block_size)
        .fold(CryptoData::new(), |acc, input_block| {
            for known_input_size in 1..crypto_block_size {
               let known_input: String = (0..known_input_size).map(|_| "A").collect();

               let mut reference_vector = input_block.to_owned().clone();
               reference_vector.truncate(known_input_size);

               let mut known_bytes = input_block.to_owned().clone();
               known_bytes.truncate(known_input_size - 1);

               let crypto_byte_map: HashMap<Vec<u8>, Vec<u8>> = (0..255)
                   .fold(HashMap::new(), |mut crypto_map, random_char| {
                       let constructed_input: CryptoData = CryptoData::new_from_str(&known_input)
                           .concat(CryptoData::new_from_vec(known_bytes.to_owned().clone()))
                           .concat(CryptoData::new_from_vec(vec![random_char]));
                       crypto_map.insert(
                           constructed_input.aes_128_ecb_encrypt(&random_key).to_vec(),
                           constructed_input.to_vec()
                        );
                       crypto_map
                   });

               let encrypted_reference_vector: Vec<u8> = CryptoData::new_from_str(&known_input)
                   .concat(CryptoData::new_from_vec(reference_vector.to_owned().clone()))
                   .aes_128_ecb_encrypt(&random_key)
                   .to_vec();

               let result = crypto_byte_map.get(&encrypted_reference_vector);
               println!("{:?}", result);

               // for (plain, crypto) in &crypto_byte_map {
               //     println!("plain: {:?}", plain);
               //     println!("crypto: {:?}", crypto);
               // }
            }
            acc
        });

    CryptoData::new()
}
