use super::super::utils::crypto_data::CryptoData;

pub fn repeating_key_xor(input_str: &str, key: &str) -> CryptoData {

    let input_crypto = CryptoData::new_from_str(input_str);
    let key_crypto = CryptoData::new_from_str(key);

    input_crypto.xor(key_crypto).to_hex()

}
