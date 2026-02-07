use thiserror::Error;

//pub mod nested;
pub mod cache;
pub mod dynamic;
pub mod generable;
pub mod primitives;
#[derive(Debug, Error)]
pub enum SchemaError {}
