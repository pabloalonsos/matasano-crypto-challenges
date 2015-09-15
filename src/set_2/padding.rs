use super::super::utils::crypto_data::CryptoData;

pub fn padding(input_string: &str, padding: u8) -> CryptoData {
    CryptoData::new_from_str(input_string)
        .padding(padding)
}
