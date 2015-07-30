
//extern crate rustc_serialize as serialize;

use std::char;
//use self::serialize::base64::{STANDARD, ToBase64};
//use self::serialize::hex::ToHex;

fn convert_hex_to_base64(hex_input: &str) {

    let hex_string: String = hex_input.to_string();
    let mut base64: String = String::new();

    for item in (*hex_string.as_bytes()).iter() {
       // base64 = base64 + char::from_u32(*item).to_string();
        let item_as_char: Option<char> = char::from_u32(*item as u32);
        let item_as_string: String = match item_as_char {
            Some(item_as_char) => item_as_char.to_string(),
            None => String::new()
        };

        let item_as_base64 = match *item {
            0...25 => 1,
            26...51 => 2,
            52...61 => 3,
            62 => 4,
            63 => 5,
            _ => 0
        };

        println!("{:?}", item_as_base64);
        base64 = base64 + &item_as_string;
        //println!("{:?}", base64);
    }
//    let res = result.iter().map(| &item | {
//        println!("HODOOOOOOR");
//        println!("{:?}", item);
//        item;
//    });

//    for n in &result[..] {
        //println!("{:?}", *n);
 //   }
}

pub fn set_1() {
    println!("");
    println!("Set 1:");
    println!("");

    println!("Exercise 1: http://cryptopals.com/sets/1/challenges/1/");
    convert_hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

}
