//extern crate rustc_serialize as serialize;
use super::super::utils::crypto_data::CryptoData;

pub fn decrypt_xor_char(hex_input: &str) -> (i32, CryptoData, CryptoData) {

    let hex_crypto_input: CryptoData = CryptoData::new_from_str(hex_input);

    hex_crypto_input.from_hex().decrypt_single_xor()

}
