#[cfg(test)]
pub mod testing;
use codegen::DynamicGenerable;
use generable::{
    DynamicGenerable,
    dynamic::schema::{DynamicSchema, DynamicStruct, UnionVariant},
};
#[derive(Debug, serde::Serialize, DynamicGenerable)]
pub enum Directions {
    North,
    South,
    West,
    East,
}

#[derive(Debug, serde::Serialize, DynamicGenerable)]
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

fn main() {
    println!("Hello, world!");
    let adr = Address::Empty;
    //let adr = Address::Coordinate { x: 0.0, y: 0.0 };
    //let s = <String as DynamicGenerable>::dynamic_schema();
    let js = serde_json::to_string_pretty(&adr).unwrap();
    let dir: DynamicSchema<&'static str> =
        DynamicSchema::Enum(vec!["North", "East", "South", "North"]);
    let dv = dir.to_value().unwrap();
    println!("{dv}");
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
    let other_schema = Address::dynamic_schema().to_value().unwrap();
    let schema_as_expected = schema == Address::dynamic_schema();
    let node = schema.to_value().unwrap();
    println!("{other_schema:#?}\n{node:#?}");

    //let njs = serde_json::to_string_pretty(&node).unwrap();
    println!("{js}");
    // println!("{njs}");

    let autogen_schema = Directions::dynamic_schema();
    println!("{autogen_schema:?}");
    println!("Schema as expected: {schema_as_expected}");
}
