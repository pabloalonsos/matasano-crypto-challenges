// Challenge 1
mod hex_to_base64;
// Challenge 2
mod fixed_xor;
// Challenge 3
mod decrypt_xor_char;
// Challenge 4
mod detect_char_xor;
// Challenge 5
mod repeating_key_xor;
// Challenge 6
mod break_repeating_key_xor;
// Challenge 7
mod aes_ecb_mode;
// Challenge 8
mod detect_aes_ecb;

use self::hex_to_base64::hex_to_base64 as hex_to_base64;
use self::fixed_xor::fixed_xor as fixed_xor;
use self::decrypt_xor_char::decrypt_xor_char as decrypt_xor_char;
use self::detect_char_xor::detect_char_xor as detect_char_xor;
use self::repeating_key_xor::repeating_key_xor as repeating_key_xor;
use self::break_repeating_key_xor::break_repeating_key_xor as break_repeating_key_xor;
use self::aes_ecb_mode::aes_ecb_mode as aes_ecb_mode;
use self::detect_aes_ecb::detect_aes_ecb as detect_aes_ecb;

use super::utils::crypto_data::CryptoData;

pub fn set_1() {
    println!("Exercise 1: Convert HEX to Base64 (http://cryptopals.com/sets/1/challenges/1/)");
    let input_1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result_1 = hex_to_base64(input_1);
    println!("Result >>> {}", result_1);
    println!("");

    println!("Exercise 2: Fixed XOR (http://cryptopals.com/sets/1/challenges/2/)");
    let input_2_1 = "1c0111001f010100061a024b53535009181c";
    let input_2_2 = "686974207468652062756c6c277320657965";
    let result_2 = fixed_xor(input_2_1, input_2_2);
    println!("Result >>> {}", result_2);
    println!("");

    println!("Exercise 3: Single-byte XOR cipher (http://cryptopals.com/sets/1/challenges/3/)");
    let (score_3, key_3, decrypted_text_3): (i32, CryptoData, CryptoData) = decrypt_xor_char("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Result >>> Score: {}; Key: '{}'; Decrypted: {}", score_3, key_3, decrypted_text_3);
    println!("");

    println!("Exercise 4: Detect single-character XOR (http://cryptopals.com/sets/1/challenges/4/)");
    let (score_4, key_4, decrypted_text_4): (i32, CryptoData, CryptoData) = detect_char_xor("./assets/4.txt");
    println!("Result >>> Score: {}; Key: '{}'; Decrypted: {}", score_4, key_4, decrypted_text_4);
    println!("");

    println!("Exercise 5: Implementing Repeating-key XOR (http://cryptopals.com/sets/1/challenges/5/)");
    let input_5 = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
    let key_5 = "ICE";
    let result_5: CryptoData = repeating_key_xor(input_5, key_5);
    println!("Result >>> {}", result_5);
    println!("");

    println!("Exercise 6: Break Repeating-key XOR (http://cryptopals.com/sets/1/challenges/6/)");
    let result_6: CryptoData = break_repeating_key_xor("./assets/6.txt");
    println!("Result >>> {}", result_6);
    println!("");

    println!("Exercise 7: AES in ECB mode (http://cryptopals.com/sets/1/challenges/7/)");
    let result_7: CryptoData = aes_ecb_mode("./assets/7.txt");
    println!("Result >>> {}", result_7);
    println!("");

    println!("Exercise 8: Detect AES in ECB (http://cryptopals.com/sets/1/challenges/8/)");
    let result_8: CryptoData = detect_aes_ecb("./assets/8.txt");
    println!("Result >>> {}", result_8);
    println!("");
}
