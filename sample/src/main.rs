use makromin::{makrolead, add_serde};
use serde::{Serialize, Deserialize};

#[makrolead]
struct Test{
    a: i32,
}

add_serde!("src/test.rs"); //(env!("OUT_DIR")/"test.rs");

fn main() {
    println!("Hello, world!");

    let a1 = A1{a:1};
    /*
    let a2 = A2{b:2};
    let a3 = Test{a:3};
    println!("{}", serde_json::to_string(&a1).unwrap());
    println!("{}", serde_json::to_string(&a2).unwrap());
    println!("{}", serde_json::to_string(&a3).unwrap());
    */

}


