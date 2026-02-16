use std::collections::HashMap;

use crate::dynamic::schema::UnionVariant;
use crate::dynamic::{Array, False, Object, String, True};
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;
///`Node<Key>` represents a JSON-Schema AST-Element with generic string keys
#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Node<Key: AsRef<str> + Serialize> {
    #[serde(rename_all = "camelCase")]
    OneOf {
        one_of: Vec<Self>,
    },
    Enum {
        r#type: String,
        r#enum: Vec<Key>,
    },
    Primitive {
        r#type: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<serde_json::Number>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<serde_json::Number>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<Key>,
    },
    #[serde(rename_all = "camelCase")]
    Struct {
        r#type: Object,
        properties: HashMap<Key, Self>,
        required: Vec<Key>,
        additional_properties: False,
    },
    Header {
        r#type: Object,
        properties: HashMap<Key, Self>,
        required: Vec<Key>,
        strict: True,
    },
    Array {
        r#type: Array,
        items: Box<Self>,
    },
}
///Convenience constructor functions for Node
impl<Key: AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str> + Debug> Node<Key> {
    pub fn header(properties: HashMap<Key, Self>, required: Vec<Key>) -> Self {
        Self::Header {
            r#type: Object,
            properties,
            required,
            strict: True,
        }
    }
    ///This function takes Node::Struct or Node::Header and turns it into Node::Header
    ///Other Node variants are passed through
    ///If you need to handle passthrough use try_as_header()
    pub fn as_header(self) -> Self {
        match self {
            Self::Struct {
                r#type,
                properties,
                required,
                ..
            } => Self::Header {
                r#type,
                properties,
                required,
                strict: True,
            },
            other => other,
        }
    }
    ///This function takes Node::Struct and turns it into Node::Header
    ///Node::Header doesnt do anything, other variants result in error
    ///Use this function if you need to explicitly handle not having a header
    pub fn try_as_header(self) -> Result<Self, Self> {
        match self {
            Self::Header {
                r#type,
                properties,
                required,
                strict,
            } => Ok(Self::Header {
                r#type,
                properties,
                required,
                strict,
            }),
            Self::Struct {
                r#type,
                properties,
                required,
                ..
            } => Ok(Self::Header {
                r#type,
                properties,
                required,
                strict: True,
            }),
            other => Err(other),
        }
    }
    pub fn r#enum(variants: Vec<Key>) -> Self {
        Node::Enum {
            r#type: String,
            r#enum: variants,
        }
    }
    pub fn number(
        r#type: &'static str,
        min: impl Into<serde_json::Number>,
        max: impl Into<serde_json::Number>,
    ) -> Self {
        Self::Primitive {
            r#type,
            minimum: Some(min.into()),
            maximum: Some(max.into()),
            format: None,
        }
    }
    pub fn primitive(r#type: &'static str) -> Self {
        Self::Primitive {
            r#type,
            minimum: None,
            maximum: None,
            format: None,
        }
    }
    pub fn union(variants: Vec<UnionVariant<Key>>) -> Self {
        let mut one_of = Vec::with_capacity(variants.capacity());
        for v in variants {
            let node = v.node();
            one_of.push(node);
        }
        Self::OneOf { one_of }
    }
    /*pub fn one_of(variants: Vec<Self>) -> Self {
        Self::OneOf { one_of: variants }
    }*/
    pub fn array(items: Self) -> Self {
        Self::Array {
            r#type: Array,
            items: Box::new(items),
        }
    }
    pub fn object(properties: HashMap<Key, Self>, required: Vec<Key>) -> Self {
        Self::Struct {
            r#type: Object,
            properties,
            required,
            additional_properties: False,
        }
    }
}
