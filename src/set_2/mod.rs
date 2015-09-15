// Challenge 9
mod padding;

use self::padding::padding as padding;

use super::utils::crypto_data::CryptoData;

pub fn set_2() {
    println!("Exercise 9: Implement PKCS#7 padding (http://cryptopals.com/sets/2/challenges/9/)");
    let input_9_string = "YELLOW SUBMARINE";
    let input_9_padding: u8 = 20;
    let result_9: CryptoData = padding(input_9_string, input_9_padding);
    println!("Result >>> {:?}", result_9);
    println!("");
}
