extern crate rustc_serialize;
extern crate shapes;
use shapes::*;
use rustc_serialize::json::{ self, ToJson };

fn main() {
    let test = Shape::Circle {
        radius: 0.1,
        x: 0.2,
        y: 0.4,
    };
    assert_eq!(&json::decode::<Shape>(&test.to_json().to_string()).unwrap(), &test)
}
