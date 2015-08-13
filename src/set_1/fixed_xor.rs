use super::super::utils::crypto_data::CryptoData;

pub fn fixed_xor(input_1: &str, input_2: &str) -> String {

    let input_1_crypto: CryptoData = CryptoData::new_from_str(input_1).from_hex();
    let input_2_crypto: CryptoData = CryptoData::new_from_str(input_2).from_hex();

    input_1_crypto
        .xor(input_2_crypto)
        .to_hex()
        .to_string()

}
