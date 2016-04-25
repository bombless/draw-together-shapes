extern crate rustc_serialize;
use rustc_serialize::json::{ ToJson, Json };
use rustc_serialize::{ Decodable, Decoder };
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle {
        radius: f64,
        x: f64,
        y: f64,
    }
}

impl ToJson for Shape {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        match self {
            &Shape::Circle {
                radius,
                x,
                y,
            } => {
                d.insert("type".to_string(), "circle".to_json());
                d.insert("radius".to_string(), radius.to_json());
                d.insert("x".to_string(), x.to_json());
                d.insert("y".to_string(), y.to_json());
                Json::Object(d)
            }
        }
    }
}

impl Decodable for Shape {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("Shape", 1, |d| {
            Ok(Shape::Circle {
                radius: try!(d.read_struct_field("radius", 0, Decodable::decode)),
                x: try!(d.read_struct_field("x", 0, Decodable::decode)),
                y: try!(d.read_struct_field("y", 0, Decodable::decode)),                
            })
        })
    }
}

