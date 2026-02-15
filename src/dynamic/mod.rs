use std::fmt::Debug;

use crate::dynamic::schema::DynamicSchema;
pub mod description;
pub mod gemini;
pub mod node;
pub mod schema;

/*macro_rules! key_type {
    ($name:ident,) => {
        impl<Key:AsRef<str> + Serialize + Clone + Hash + Eq + From<&'static str>> $name {}
    };
}*/

/*
* #[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Optional. The type of the data.
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    /// Optional. The format of the data.
    /// Supported formats:
    /// for NUMBER type: "float", "double"
    /// for INTEGER type: "int32", "int64"
    /// for STRING type: "email", "byte", etc
    #[prost(string, tag = "7")]
    pub format: ::prost::alloc::string::String,
    /// Optional. The title of the Schema.
    #[prost(string, tag = "24")]
    pub title: ::prost::alloc::string::String,
    /// Optional. The description of the data.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Indicates if the value may be null.
    #[prost(bool, tag = "6")]
    pub nullable: bool,
    /// Optional. Default value of the data.
    #[prost(message, optional, tag = "23")]
    pub default: ::core::option::Option<super::super::super::protobuf::Value>,
    /// Optional. SCHEMA FIELDS FOR TYPE ARRAY
    /// Schema of the elements of Type.ARRAY.
    #[prost(message, optional, boxed, tag = "2")]
    pub items: ::core::option::Option<::prost::alloc::boxed::Box<Schema>>,
    /// Optional. Minimum number of the elements for Type.ARRAY.
    #[prost(int64, tag = "21")]
    pub min_items: i64,
    /// Optional. Maximum number of the elements for Type.ARRAY.
    #[prost(int64, tag = "22")]
    pub max_items: i64,
    /// Optional. Possible values of the element of primitive type with enum
    /// format. Examples:
    ///
    /// 1. We can define direction as :
    ///    {type:STRING, format:enum, enum:\["EAST", NORTH", "SOUTH", "WEST"\]}
     /// 1. We can define apartment number as :
     ///    {type:INTEGER, format:enum, enum:\["101", "201", "301"\]}
     #[prost(string, repeated, tag = "9")]
     pub r#enum: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
     /// Optional. SCHEMA FIELDS FOR TYPE OBJECT
     /// Properties of Type.OBJECT.
     #[prost(map = "string, message", tag = "3")]
     pub properties: ::std::collections::HashMap<::prost::alloc::string::String, Schema>,
     /// Optional. The order of the properties.
     /// Not a standard field in open api spec. Only used to support the order of
     /// the properties.
     #[prost(string, repeated, tag = "25")]
     pub property_ordering: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
     /// Optional. Required properties of Type.OBJECT.
     #[prost(string, repeated, tag = "5")]
     pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
     /// Optional. Minimum number of the properties for Type.OBJECT.
     #[prost(int64, tag = "14")]
     pub min_properties: i64,
     /// Optional. Maximum number of the properties for Type.OBJECT.
     #[prost(int64, tag = "15")]
     pub max_properties: i64,
     /// Optional. SCHEMA FIELDS FOR TYPE INTEGER and NUMBER
     /// Minimum value of the Type.INTEGER and Type.NUMBER
     #[prost(double, tag = "16")]
     pub minimum: f64,
     /// Optional. Maximum value of the Type.INTEGER and Type.NUMBER
     #[prost(double, tag = "17")]
     pub maximum: f64,
     /// Optional. SCHEMA FIELDS FOR TYPE STRING
     /// Minimum length of the Type.STRING
     #[prost(int64, tag = "18")]
     pub min_length: i64,
     /// Optional. Maximum length of the Type.STRING
     #[prost(int64, tag = "19")]
     pub max_length: i64,
     /// Optional. Pattern of the Type.STRING to restrict a string to a regular
     /// expression.
     #[prost(string, tag = "20")]
     pub pattern: ::prost::alloc::string::String,
     /// Optional. Example of the object. Will only populated when the object is the
     /// root.
     #[prost(message, optional, tag = "4")]
     pub example: ::core::option::Option<super::super::super::protobuf::Value>,
     /// Optional. The value should be validated against any (one or more) of the
     /// subschemas in the list.
     #[prost(message, repeated, tag = "11")]
     pub any_of: ::prost::alloc::vec::Vec<Schema>,
     /// Optional. Can either be a boolean or an object; controls the presence of
     /// additional properties.
     #[prost(message, optional, tag = "26")]
     pub additional_properties: ::core::option::Option<
         super::super::super::protobuf::Value,
     >,
     /// Optional. Allows indirect references between schema nodes. The value should
     /// be a valid reference to a child of the root `defs`.
     ///
     /// For example, the following schema defines a reference to a schema node
     /// named "Pet":
     ///
     /// type: object
     /// properties:
     /// pet:
     /// ref: #/defs/Pet
     /// defs:
     /// Pet:
     /// type: object
     /// properties:
     /// name:
     /// type: string
     ///
     /// The value of the "pet" property is a reference to the schema node
     /// named "Pet".
     /// See details in
     /// <https://json-schema.org/understanding-json-schema/structuring>
     #[prost(string, tag = "27")]
     pub r#ref: ::prost::alloc::string::String,
     /// Optional. A map of definitions for use by `ref`
     /// Only allowed at the root of the schema.
     #[prost(map = "string, message", tag = "28")]
     pub defs: ::std::collections::HashMap<::prost::alloc::string::String, Schema>,
 }
 /// Type contains the list of OpenAPI data types as defined by
 /// <https://swagger.io/docs/specification/data-models/data-types/>
 #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
 #[repr(i32)]
 pub enum Type {
     /// Not specified, should not be used.
     Unspecified = 0,
     /// OpenAPI string type
     String = 1,
     /// OpenAPI number type
     Number = 2,
     /// OpenAPI integer type
     Integer = 3,
     /// OpenAPI boolean type
     Boolean = 4,
     /// OpenAPI array type
     Array = 5,
     /// OpenAPI object type
     Object = 6,
 }
 */

pub trait DynamicGenerable {
    const REQUIRED: bool = true;
    fn dynamic_schema() -> DynamicSchema<&'static str>;
}
impl<T: DynamicGenerable> DynamicGenerable for Vec<T> {
    fn dynamic_schema() -> DynamicSchema<&'static str> {
        DynamicSchema::Array(Box::new(T::dynamic_schema()))
    }
}
impl<T: DynamicGenerable> DynamicGenerable for Option<T> {
    const REQUIRED: bool = false;
    fn dynamic_schema() -> DynamicSchema<&'static str> {
        DynamicSchema::Option(Box::new(T::dynamic_schema()))
    }
}
macro_rules! impl_dynamic_generable_for_ints {
    ($($t:ty),* $(,)?) => {
        $(
            impl DynamicGenerable for $t {
                fn dynamic_schema() -> DynamicSchema<&'static str> {
                    DynamicSchema::Integer
                }
            }
        )*
    };
}
impl_dynamic_generable_for_ints!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);
macro_rules! impl_dynamic_generable_for_floats {
    ($($t:ty),* $(,)?) => {
        $(
            impl DynamicGenerable for $t {
                fn dynamic_schema() -> DynamicSchema<&'static str> {
                    DynamicSchema::Number
                }
            }
        )*
    };
}
impl_dynamic_generable_for_floats!(f64, f32);
macro_rules! impl_dynamic_generable_for_str {
    ($($t:ty),* $(,)?) => {
        $(
            impl DynamicGenerable for $t {
                fn dynamic_schema() -> DynamicSchema<&'static str> {
                    DynamicSchema::String
                }
            }
        )*
    };
}
impl_dynamic_generable_for_str!(
    std::string::String,
    &str,
    std::sync::Arc<str>,
    std::rc::Rc<str>,
);

macro_rules! static_value_type {
    ($name:ident, $value:expr) => {
        ///Autogenerated static serializable type string
        #[derive(Debug)]
        pub struct $name;
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str($value)
            }
        }
    };
}
static_value_type!(Object, "object");
static_value_type!(String, "string");
static_value_type!(Array, "array");
static_value_type!(Boolean, "boolean");
static_value_type!(Number, "number");
static_value_type!(Integer, "integer");
#[derive(Debug)]
pub struct False;
impl serde::Serialize for False {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(false)
    }
}
#[derive(Debug)]
pub struct True;
impl serde::Serialize for True {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(true)
    }
}
