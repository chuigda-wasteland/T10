use crate::data::{StaticWrapper, Value};
use std::mem::MaybeUninit;

mod ramped;
mod oxtank;

pub trait Intake<'a> {
    fn allocate<T>(&mut self) -> StaticWrapper<T>;
    fn force_collect(&mut self);
    fn add_stack(&mut self, stack: &'a Vec<MaybeUninit<Value>>);
    fn mark_permanent(&mut self, value: Value);
}
