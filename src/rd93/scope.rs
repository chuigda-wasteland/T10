use crate::data::Value;

pub struct Scope {
    pub values: Box<[Value]>,
    pub ret_addr: usize,
    pub ret_value_loc: usize
}

impl Scope {
    pub fn new(value_count: usize, ret_addr: usize) -> Self {
        let mut values = Vec::new();
        values.resize(value_count, Value::null());
        Self {
            values: values.into_boxed_slice(),
            ret_addr,
            ret_value_loc: 0
        }
    }

    #[cfg(not(debug_assertions))]
    pub unsafe fn get_value(&self, idx: usize) -> Value {
        *self.values.get_unchecked(idx)
    }

    #[cfg(debug_assertions)]
    pub unsafe fn get_value(&self, idx: usize) -> Value {
        self.values[idx]
    }

    #[cfg(not(debug_assertions))]
    pub unsafe fn set_value(&mut self, idx: usize, value: Value) {
        *self.values.get_unchecked_mut(idx) = value;
    }

    #[cfg(debug_assertions)]
    pub unsafe fn set_value(&mut self, idx: usize, value: Value) {
        self.values[idx] = value;
    }
}
