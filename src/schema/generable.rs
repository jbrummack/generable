use std::collections::HashMap;

use crate::schema::primitives::SchemaObject;

pub trait Generable {
    fn properties() -> Option<HashMap<&'static str, SchemaObject>> {
        None //HashMap::new()
    }
    fn required_fields() -> Option<Vec<&'static str>> {
        None
    }
    const REQUIRED: bool = true;
    //const FNAME: &'static str;
    //const REQUIRED: Option<&'static str> = Some(Self::FNAME);
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::object(
            description,
            Self::TNAME,
            Self::properties(),
            Self::required_fields(),
        )
    }
    const TNAME: &'static str = "object";

    /*fn tname() -> &'static str {
        "object"
    }*/
}
impl<T: Generable> Generable for Vec<T> {
    fn schema(description: Option<&'static str>) -> SchemaObject {
        let child_schema = T::schema(None);
        SchemaObject::array(description, child_schema)
    }
}
impl<T: Generable> Generable for Option<T> {
    fn properties() -> Option<HashMap<&'static str, SchemaObject>> {
        T::properties()
    }
    fn schema(description: Option<&'static str>) -> SchemaObject {
        T::schema(description)
    }
    const TNAME: &'static str = T::TNAME;
    const REQUIRED: bool = false;
    //const REQUIRED: Option<&'static str> = None;

    /*fn required() -> Option<&'static str> {
        None
    }*/
}

/*impl<T: GenSchema> GenSchema for Vec<T> {
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::array(description, T::schema(None))
    }
}
impl GenSchema for String {
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::string(description)
    }
}

impl GenSchema for bool {
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::boolean(description)
    }
}*/

impl Generable for bool {
    const TNAME: &'static str = "bool";
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::primitive_object(
            description,
            Self::TNAME,
            Self::properties(),
            Self::required_fields(),
        )
    }
}
impl Generable for String {
    const TNAME: &'static str = "string";
    fn schema(description: Option<&'static str>) -> SchemaObject {
        SchemaObject::primitive_object(
            description,
            Self::TNAME,
            Self::properties(),
            Self::required_fields(),
        )
    }
}
macro_rules! impl_integer_generable {
    ($($ty:ty),+) => {
        $(

            impl Generable for $ty {
                const TNAME: &'static str = "integer";
                fn schema(description: Option<&'static str>) -> SchemaObject {
                    SchemaObject::primitive_object(
                        description,
                        Self::TNAME,
                        Self::properties(),
                        Self::required_fields(),
                    )
                }
            }
        )+
    };
}
// Use it like this:
impl_integer_generable!(i8, i16, i32, i64, u8, u16, u32, u64, usize);

macro_rules! impl_number_generable {
    ($($ty:ty),+) => {
        $(
            impl Generable for $ty {
                const TNAME: &'static str = "number";
                fn schema(description: Option<&'static str>) -> SchemaObject {
                    SchemaObject::primitive_object(
                        description,
                        Self::TNAME,
                        Self::properties(),
                        Self::required_fields(),
                    )
                }
            }

        )+
    };
}
impl_number_generable!(f32, f64);
