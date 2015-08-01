// Challenge 1
mod hex_to_base64;
// Challenge 2
mod fixed_xor;
// Challenge 3
mod decrypt_xor_char;

use self::hex_to_base64::hex_to_base64 as hex_to_base64;
use self::fixed_xor::fixed_xor as fixed_xor;
use self::decrypt_xor_char::decrypt_xor_char as decrypt_xor_char;

pub fn set_1() {
    println!("");
    println!("Set 1:");
    println!("");

    println!("Exercise 1: Convert HEX to Base64 (http://cryptopals.com/sets/1/challenges/1/)");
    let result_1 = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Result >>> {}", result_1);
    println!("");

    println!("Exercise 2: Fixed XOR (http://cryptopals.com/sets/1/challenges/2/)");
    let input_1 = "1c0111001f010100061a024b53535009181c";
    let input_2 = "686974207468652062756c6c277320657965";
    let result_2 = fixed_xor(input_1, input_2);
    println!("Result >>> {}", result_2);
    println!("");

    println!("Exercise 3: Single-byte XOR cipher (http://cryptopals.com/sets/1/challenges/3/)");
    let result_3 = decrypt_xor_char("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("Result >>> {}", result_3);
    println!("");

}