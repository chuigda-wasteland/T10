/*

use crate::data::{Value, ValueType, DynBase};

// We cannot use a single pointer since we'd like to support value types.
// `Ptr` always use heap allocation!

pub trait ValueFromRust<'a, T: 'a> {
    unsafe fn from_any(t: T) -> Result<Value<'a>, String>;
}

pub trait ValueFromRustImpl<'a, T: 'a> {
    unsafe fn from_any_impl(t: T) -> Result<Value<'a>, String>;
}

pub trait ValueFromRustImpl2<'a, T: 'a> {
    unsafe fn from_any_impl2(t: T) -> Result<Value<'a>, String>;
}

impl<'a, T: 'a> ValueFromRust<'a, T> for () {
    default unsafe fn from_any(t: T) -> Result<Value<'a>, String> {
        <() as ValueFromRustImpl<'a, T>>::from_any_impl(t)
    }
}

impl<'a, T: 'a> ValueFromRust<'a, Option<T>> for () {
    default unsafe fn from_any(t: Option<T>) -> Result<Value<'a>, String> {
        if let Some(t) = t {
            <() as ValueFromRust<T>>::from_any(t)
        } else {
            Ok(Value::null_value_type(ValueType::AnyType))
        }
    }
}

impl<'a, T: 'a> ValueFromRust<'a, &'a Option<T>> for () {
    unsafe fn from_any(t: &'a Option<T>) -> Result<Value<'a>, String> {
        <() as ValueFromRust<Option<&'a T>>>::from_any(t.as_ref())
    }
}

impl<'a, T: 'a> ValueFromRust<'a, &'a mut Option<T>> for () {
    unsafe fn from_any(t: &'a mut Option<T>) -> Result<Value<'a>, String> {
        <() as ValueFromRust<Option<&'a mut T>>>::from_any(t.as_mut())
    }
}

impl<'a, T: 'a> ValueFromRustImpl<'a, T> for () {
    default unsafe fn from_any_impl(t: T) -> Result<Value<'a>, String> {
        <() as ValueFromRustImpl2<T>>::from_any_impl2(t)
    }
}

impl<'a, T: 'a> ValueFromRustImpl<'a, &'a T> for () {
    unsafe fn from_any_impl(t: &'a T) -> Result<Value<'a>, String> {
        Ok(todo!())
    }
}

impl<'a, T: 'a> ValueFromRustImpl<'a, &'a mut T> for () {
    unsafe fn from_any_impl(t: &'a mut T) -> Result<Value<'a>, String> {
        unimplemented!()
    }
}

impl<'a, T: 'a> ValueFromRustImpl2<'a, T> for () {
    default unsafe fn from_any_impl2(t: T) -> Result<Value<'a>, String> {
        unimplemented!()
    }
}

impl<'a, T: 'a + Copy> ValueFromRustImpl2<'a, T> for () {
    unsafe fn from_any_impl2(t: T) -> Result<Value<'a>, String> {
        unimplemented!()
    }
}
*/