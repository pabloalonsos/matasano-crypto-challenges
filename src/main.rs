mod set_1;
mod utils;

use self::set_1::set_1 as set_1;
//use utils::crypto_data::{CryptoData};

fn main() {
    println!("==============");
    println!("Section One");
    println!("==============");
    set_1();
    //let t: CryptoData = CryptoData::new("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes().to_vec());
    //println!("{}", t.to_base64());
}
