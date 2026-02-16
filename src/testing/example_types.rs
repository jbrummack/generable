use generable::derive::DynamicGenerable;
//use generable::dynamic::schema::{DynamicStruct, UnionVariant};
#[derive(Debug, serde::Serialize, serde::Deserialize, DynamicGenerable)]
pub enum Address {
    Empty,
    Invalid,
    Coordinate {
        x: f64,
        y: f64,
    },
    Address {
        country: String,
        number: u32,
        street: String,
    },
}
#[derive(Debug, serde::Serialize, serde::Deserialize, DynamicGenerable)]
pub struct TestStruct {
    v1: u8,
    v2: String,
}
