//https://huggingface.co/Qwen/Qwen3-VL-2B-Instruct/resolve/main/tokenizer.json?download=true
//https://huggingface.co/Qwen/Qwen3-VL-2B-Instruct/resolve/main/vocab.json?download=true
pub mod example_types;
pub mod sampler;
#[cfg(test)]
mod tests {
    use crate::{
        DynamicGenerable,
        dynamic::schema::{DynamicSchema, DynamicStruct, UnionVariant},
        testing::{
            example_types::Address,
            sampler::{Sampler, ascii_vocab},
        },
    };
    #[test]
    fn check_json_correctness() {
        let node = Address::dynamic_schema().to_value().unwrap();
        let validator = jsonschema::validator_for(&node).unwrap();
        let vocab = ascii_vocab();
        let mut sampler = Sampler::new(node, &vocab);
        let mut output = String::new();
        while let Some(next) = sampler.next() {
            //print!("{next}")
            output.push_str(&next);
        }
        let val: serde_json::Value = serde_json::from_str(&output).unwrap();
        let evaluation = validator.evaluate(&val).flag();
        assert!(evaluation.valid);
        let _deserialized: Address =
            serde_json::from_value(val).expect("Fuzztested value cant be deserialized");
    }
    #[test]
    fn check_expected_schema() {
        let expect: DynamicSchema<&'static str> = DynamicSchema::Union(vec![
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
        assert_eq!(expect, Address::dynamic_schema())
    }
}
