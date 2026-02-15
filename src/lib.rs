//! # Generable
//!
//! Generable is a crate for generating JSON-Schema efficiently and generically.
//!
//! Generable data structures
//!
//! ## Design
//!
//! The Generable derive macro generates a dynamic schema which can be converted
//! into a JSON-Schema or other Schemas like a gemini gRPC protobuf::Value.
//! Because the schema of an object isnt subject to change we can generate it once
//! and cache it as a static value after that to avoid recomputing it again.
pub mod dynamic;
pub use dynamic::DynamicGenerable;
#[cfg(test)]
pub mod testing;
