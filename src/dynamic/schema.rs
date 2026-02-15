use std::collections::HashMap;

use crate::dynamic::node::Node;
use crate::dynamic::{False, Object, String};
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;
///DynamicStruct helper
#[derive(Debug, Clone)]
pub struct DynamicStruct<Key>(pub HashMap<Key, DynamicSchema<Key>>)
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug;
impl<Key> DynamicStruct<Key>
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug,
{
    //Clone and output required fields
    pub fn required(&self) -> Vec<Key> {
        self.0
            .iter()
            .filter_map(|(k, v)| {
                if v.is_required() {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect()
    }
    /*pub fn to_union_variant(self, variant_name: &'static str) -> Node<Key> {
        let required = self.required();
        let mut properties = self.properties();
        let type_text: Key = Key::from("type");
        properties.insert(
            type_text,
            Node::Primitive {
                r#type: "string",
                minimum: None,
                maximum: None,
            },
        );
        Node::object(properties, required)
    }*/
    pub fn into_enum_variant(self, name: Key) -> Node<Key> {
        Node::Struct {
            r#type: Object,
            properties: [(name.clone(), self.to_object_node())].into(),
            required: vec![name],
            additional_properties: False,
        }
    }
    pub fn to_header_node(self) -> Node<Key> {
        let required = self.required();
        let properties = self.properties();
        Node::header(properties, required)
    }
    pub fn to_object_node(self) -> Node<Key> {
        let required = self.required();
        let properties = self.properties();
        Node::object(properties, required)
    }

    pub fn properties(self) -> HashMap<Key, Node<Key>> {
        let mut result = HashMap::with_capacity(self.0.len());
        for (k, v) in self.0 {
            result.insert(k, v.to_node());
        }
        result
    }
}
#[derive(Debug, Clone)]
pub enum UnionVariant<Key>
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug,
{
    Struct(Key, DynamicStruct<Key>),
    Enum(Key),
}
impl<Key> UnionVariant<Key>
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug,
{
    pub fn node(self) -> Node<Key> {
        match self {
            UnionVariant::Struct(k, dynamic_struct) => dynamic_struct.into_enum_variant(k),
            UnionVariant::Enum(key) => Node::Enum {
                r#type: String,
                r#enum: vec![key],
            },
        }
    }
}
///DynamicSchema represents a machine generated intermediate AST that can be converted into different structured schemas (e.g. JSON-Schema)
#[derive(Debug, Clone)]
pub enum DynamicSchema<Key>
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug,
{
    Integer {
        min: i64,
        max: u64,
    },
    Number {
        min: f64,
        max: f64,
    },
    Bool,
    String,
    Struct(DynamicStruct<Key>),
    ///Externally tagged union (serde_jsons default behavior)
    Union(Vec<UnionVariant<Key>>),
    Enum(Vec<Key>),
    Option(Box<Self>),
    Array(Box<Self>),
}
impl<Key> DynamicSchema<Key>
where
    Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug,
{
    pub fn is_required(&self) -> bool {
        if let Self::Option(_) = self {
            false
        } else {
            true
        }
    }
    ///Returns a JSON-Schema-Value
    pub fn to_string(self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self.to_schema())
    }
    pub fn to_string_pretty(self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string_pretty(&self.to_schema())
    }
    ///Returns a JSON-Schema-Value
    pub fn to_value(self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self.to_schema())
    }
    ///Returns a JSON-Schema
    pub fn to_schema(self) -> Node<Key> {
        self.to_node().as_header()
    }
    ///convert to internal JSON-Schema AST without corrected header
    ///use to_schema() for non internal use cases
    pub fn to_node(self) -> Node<Key> {
        match self {
            DynamicSchema::Integer { min, max } => Node::number("integer", min, max),
            DynamicSchema::Number { min, max } => Node::number(
                "number",
                serde_json::Number::from_f64(min).unwrap_or(0.into()),
                serde_json::Number::from_f64(max).unwrap_or(0.into()),
            ),
            DynamicSchema::Bool => Node::primitive("bool"),
            DynamicSchema::String => Node::primitive("string"),
            DynamicSchema::Struct(dynamic_struct) => dynamic_struct.to_object_node(),
            DynamicSchema::Union(variants) => Node::union(variants),
            DynamicSchema::Enum(items) => Node::r#enum(items),
            DynamicSchema::Option(dynamic_schema) => dynamic_schema.to_node(),
            DynamicSchema::Array(dynamic_schema) => Node::array(dynamic_schema.to_node()),
        }
    }
}
