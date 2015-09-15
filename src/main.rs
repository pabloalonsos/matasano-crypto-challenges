mod set_1;
mod set_2;
mod utils;

use self::set_1::set_1 as set_1;
use self::set_2::set_2 as set_2;

fn main() {
    println!("==============");
    println!("Section One");
    println!("==============");
    set_1();

    println!("==============");
    println!("Section Two");
    println!("==============");
    set_2();
}
