use std::mem::MaybeUninit;
use crate::data::{Value, StaticWrapper};
use crate::intake::Intake;

pub struct RampedIntake<'a> {
    stacks: Vec<&'a Vec<MaybeUninit<Value>>>,
    heap: Vec<*mut Value>
}

impl<'a> RampedIntake<'a> {
    pub fn new() -> Self {
        Self {
            stacks: Vec::new(),
            heap: Vec::new()
        }
    }
}

impl<'a> Default for RampedIntake<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Intake<'a> for RampedIntake<'a> {
    fn allocate<T>(&mut self) -> StaticWrapper<T> {
        todo!()
    }

    fn force_collect(&mut self) {
        todo!()
    }

    fn add_stack(&mut self, stack: &'a Vec<MaybeUninit<Value>>) {
        todo!()
    }

    fn mark_permanent(&mut self, value: Value) {
        todo!()
    }
}
