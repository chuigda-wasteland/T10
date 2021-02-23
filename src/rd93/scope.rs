use crate::data::Value;

pub struct Scope {
    pub values: Box<[Value]>,
    pub ret_addr: usize,
    pub ret_loc: usize
}

impl Scope {
    pub fn new(value_count: usize, ret_addr: usize) -> Self {
        let mut values = Vec::new();
        values.resize(value_count, Value::null());
        Self {
            values: values.into_boxed_slice(),
            ret_addr,
            ret_loc: 0
        }
    }
}
