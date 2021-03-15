use std::mem::MaybeUninit;
use crate::data::{StaticWrapper, Value};

mod ramped;
mod oxtank;

pub trait OxProvider<'a> {
    fn allocate<T>(&mut self, t: T) -> StaticWrapper<T>;
    fn allocate_ref<T>(&mut self, t: &T) -> StaticWrapper<T>;
    fn allocate_ref_mut<T>(&mut self, t: &mut T) -> StaticWrapper<T>;

    fn force_collect(&mut self);

    fn add_stack(&mut self, stack: &'a Vec<MaybeUninit<Value>>);

    fn enable_collect(&mut self);
    fn disable_collect(&mut self);
}
