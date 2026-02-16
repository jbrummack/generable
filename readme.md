# Generable
**Generable is a crate for generating JSON-Schema efficiently and generically.**
## Design

The Generable derive macro generates a dynamic schema which can in turn be converted into a JSON-Schema or other schemas like a gemini gRPC protobuf::Value. Because the schema of an object isnt subject to change we can generate it once and cache it as a static value after that to avoid recomputing it again.


Generable doesnt focus on perfect JSON-Schema specification correctness but on compatibility with LLM Inference Providers and ease of use. It is loosely inspired by the @Generable macro from Apples FoundationModel framework.

## Usage

**Derive a Generable type**
```rust
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
```
**generate a schema**

```rust
let schema = Address::dynamic_schema().to_string_pretty()?;
```
**build a schema at runtime**

```DynamicSchema<Key: AsRef<str>>``` works with &str, String, Arc<str>, or whatever string you need at runtime as long as it conforms to ```AsRef<str>```
```rust
let schema: DynamicSchema<&'static str> = DynamicSchema::Union(vec![
            UnionVariant::Enum("Empty"),
            UnionVariant::Enum("Invalid"),
            UnionVariant::Struct(
                "Coordinate",
                DynamicStruct([("x", f64::dynamic_schema()), ("y", f64::dynamic_schema())].into()),
            ),
            UnionVariant::Struct(
                "Address",
                DynamicStruct(
                    [
                        ("country", String::dynamic_schema()),
                        ("number", u32::dynamic_schema()),
                        ("street", String::dynamic_schema()),
                    ]
                    .into(),
                ),
            ),
        ]);
```

## Roadmap
- Support JSON-Schema descriptions (and maybe an extractor for injecting those into prompts)
- More control over Integer/Number types like a BoundedInt<const MIN: i64,const MAX: i64>. Sadly there is no way to implement float const generics.
- Support for UUIDs
- Support for Dates
- Caching traits
