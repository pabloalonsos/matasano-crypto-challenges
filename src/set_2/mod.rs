// Challenge 9
mod padding;

// Challenge 10
mod cbc_mode;

// Challenge 11
mod ecb_cbc_oracle;

use self::padding::padding as padding;
use self::cbc_mode::cbc_mode as cbc_mode;
use self::ecb_cbc_oracle::ecb_cbc_oracle as ecb_cbc_oracle;

use super::utils::crypto_data::CryptoData;

pub fn set_2() {
    println!("Exercise 9: Implement PKCS#7 padding (http://cryptopals.com/sets/2/challenges/9/)");
    let input_9_string = "YELLOW SUBMARINE";
    let input_9_padding: u8 = 20;
    let result_9: CryptoData = padding(input_9_string, input_9_padding);
    println!("Result >>> {:?}", result_9);
    println!("");

    println!("Exercise 10: Implement CBC mode (http://cryptopals.com/sets/2/challenges/10/)");
    let input_10_file = "./assets/10.txt";
    let input_10_key = "YELLOW SUBMARINE";
    let result_10: CryptoData = cbc_mode(input_10_file, input_10_key);
    println!("Result >>> {}", result_10);
    println!("");

    println!("Exercise 11: An ECB/CBC detection oracle (http://cryptopals.com/sets/2/challenges/11/)");
    let result_11: &str = ecb_cbc_oracle();
    println!("Result >>> {:?}", result_11);
    println!("");
}
