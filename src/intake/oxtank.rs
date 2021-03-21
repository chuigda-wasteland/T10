use crate::data::{Value, StaticWrapper};
use crate::intake::OxProvider;
use std::mem::MaybeUninit;

pub struct OxTank {
    heap: Vec<*mut Value>,
    capacity: Option<usize>,

    used: usize
}

impl OxTank {
    pub fn new_unlimited() -> Self {
        Self {
            heap: Vec::new(),
            capacity: None,

            used: 0
        }
    }

    pub fn new_limited(capacity: usize) -> Self {
        Self {
            heap: Vec::new(),
            capacity: Some(capacity),

            used: 0
        }
    }
}

impl Default for OxTank {
    fn default() -> Self {
        OxTank::new_unlimited()
    }
}

impl OxProvider<'_> for OxTank {
    fn allocate<T>(&mut self, _t: T) -> StaticWrapper<T> {
        unimplemented!()
    }

    fn allocate_ref<T>(&mut self, _t: &T) -> StaticWrapper<T> {
        unimplemented!()
    }

    fn allocate_ref_mut<T>(&mut self, _t: &mut T) -> StaticWrapper<T> {
        unimplemented!()
    }

    fn force_collect(&mut self) {}

    fn add_stack(&mut self, _stack: &Vec<MaybeUninit<Value>>) {}

    fn enable_collect(&mut self) {}

    fn disable_collect(&mut self) {}
}
