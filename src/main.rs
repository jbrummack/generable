use codegen::DynamicGenerable;
use generable::{
    DynamicGenerable,
    dynamic::schema::{DynamicSchema, DynamicStruct, UnionVariant},
    testing::sampler::Sampler,
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
    let node = schema.to_value().unwrap();

    //let njs = serde_json::to_string_pretty(&node).unwrap();
    println!("{js}");
    // println!("{njs}");
    let validator = jsonschema::validator_for(&node).unwrap();
    let mut sampler = Sampler::new(node);
    let mut output = String::new();
    while let Some(next) = sampler.next() {
        //print!("{next}")
        output.push_str(&next);
    }
    let val: serde_json::Value = serde_json::from_str(&output).unwrap();
    let evaluation = validator.evaluate(&val).flag();
    println!("{output}");
    println!("{evaluation:?}");
    let autogen_schema = Address::dynamic_schema();
    println!("{autogen_schema:?}");
    let autogen_schema = Directions::dynamic_schema();
    println!("{autogen_schema:?}")
}
