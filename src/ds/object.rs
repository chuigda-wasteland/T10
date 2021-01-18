use std::collections::BTreeMap;

use crate::cast::from_value::FromValue;
use crate::error::TError;
use crate::data::Value;
use crate::tyck::fusion::Fusion;
use crate::void::Void;

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

    pub fn get_field<T>(&self, _name: &str) -> Result<T, TError>
        where Void: for<'a> FromValue<'a, T>,
              Void: Fusion<T>
    {
        unimplemented!()
    }
}
