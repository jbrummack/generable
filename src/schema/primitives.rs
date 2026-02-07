use std::{collections::HashMap, sync::atomic::AtomicU64};

use serde::Serialize;

#[allow(unused)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'static str>,
    r#type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<SchemaObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<HashMap<&'static str, SchemaObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<&'static str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#enum: Option<Vec<&'static str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_of: Option<&'static str>,
}
type Id = u64;
struct IdProvider {
    ctr: Id,
}
impl IdProvider {
    fn get(&mut self) -> Id {
        self.ctr += 1;
        self.ctr
    }
}
struct DescTree {
    ctr: IdProvider,
    entry: Id,
    edges: HashMap<Id, Id>,
    data: HashMap<Id, NodeData>,
}
#[derive(Debug, Clone, Copy)]
struct NodeData {
    name: &'static str,
    desc: &'static str,
}
/*pub struct DescrTree {
    entry: &'static str,

    nodes: HashMap<&'static str, (&'static str, &'static str)>,
}
pub struct DescNode {
    name: &'static str,
    description: &'static str,
}*/
impl SchemaObject {
    pub fn set_strict(&mut self) {
        self.strict = Some(true);
    }
    fn get_desc(&self, dt: &mut DescTree) {
        if let Some(props) = self.properties.as_ref() {
            for (chname, chobj) in props {
                /*if let Some(nd) = chobj.get_node_data() {
                    let id = dt.ctr.get();
                    dt.data.insert(id, nd);
                }*/
            }
        }
    }
    fn get_node_data(&self) -> Option<NodeData> {
        Some(NodeData {
            name: self.name.as_ref()?,
            desc: self.description.as_ref()?,
        })
    }
    /*pub fn get_description(&self) -> DescrTree {
            if let Some(props) = self.properties.as_ref() else {
                DescrTree { entry: (), nodes: () }
            }
        }
    */
    pub fn set_name(&mut self, name: &'static str) {
        self.name = Some(name);
    }
    pub fn add_field(&mut self, field_name: &'static str, obj: SchemaObject) {
        if let Some(properties) = self.properties.as_mut() {
            properties.insert(field_name, obj);
        } else {
            let mut properties = HashMap::new();
            properties.insert(field_name, obj);
            self.properties = Some(properties);
        };
    }
    pub fn array(description: Option<&'static str>, items: SchemaObject) -> Self {
        Self {
            description,
            r#type: "array",
            items: Some(Box::new(items)),
            properties: None,
            strict: None,
            additional_properties: None,
            required: None,
            name: None,
            r#enum: None,
            one_of: None,
        }
    }
    pub fn primitive(description: Option<&'static str>, p: &'static str) -> Self {
        Self {
            description,
            r#type: p,
            items: None,
            properties: None,
            strict: None,
            additional_properties: None,
            required: None,
            name: None,
            r#enum: None,
            one_of: None,
        }
    }
    pub fn object(
        description: Option<&'static str>,
        p: &'static str,
        properties: Option<HashMap<&'static str, SchemaObject>>,
        required: Option<Vec<&'static str>>,
    ) -> Self {
        Self {
            description,
            r#type: p,
            items: None,
            properties,
            strict: None,
            additional_properties: Some(false), //OpenAI Compatibility, TODO: Allow turning off for less tokens when using other provider
            required,
            name: None,
            r#enum: None,
            one_of: None,
        }
    }
    pub fn primitive_object(
        description: Option<&'static str>,
        p: &'static str,
        properties: Option<HashMap<&'static str, SchemaObject>>,
        required: Option<Vec<&'static str>>,
    ) -> Self {
        Self {
            description,
            r#type: p,
            items: None,
            properties,
            strict: None,
            additional_properties: None, //OpenAI Compatibility, TODO: Allow turning off for less tokens when using other provider
            required,
            name: None,
            r#enum: None,
            one_of: None,
        }
    }
    pub fn number(description: Option<&'static str>) -> Self {
        Self::primitive(description, "number")
    }
    pub fn integer(description: Option<&'static str>) -> Self {
        Self::primitive(description, "integer")
    }
    pub fn boolean(description: Option<&'static str>) -> Self {
        Self::primitive(description, "boolean")
    }
    pub fn string(description: Option<&'static str>) -> Self {
        Self::primitive(description, "string")
    }
    pub fn header(
        description: Option<&'static str>,
        properties: HashMap<&'static str, SchemaObject>,
        strict: bool,
        additional_properties: bool,
    ) -> Self {
        Self {
            description,
            r#type: "object",
            items: None,
            properties: Some(properties),
            strict: Some(strict),
            additional_properties: Some(additional_properties),
            required: None,
            name: None,
            r#enum: None,
            one_of: None,
        }
    }
}
