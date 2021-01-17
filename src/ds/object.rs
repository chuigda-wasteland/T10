use std::collections::BTreeMap;

use crate::data::Value;

pub struct DynamicObject {
    fields: BTreeMap<String, Value>
}

impl DynamicObject {
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new()
        }
    }

    pub fn get_field_untyped(&self, name: &str) -> Option<Value> {
        self.fields.get(name).map(|x| x.clone())
    }
}
