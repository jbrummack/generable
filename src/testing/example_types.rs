use crate::DynamicGenerable;
use crate::dynamic::schema::{DynamicSchema, DynamicStruct, UnionVariant};
use codegen::DynamicGenerable;
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
