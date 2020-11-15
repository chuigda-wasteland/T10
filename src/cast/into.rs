use crate::data::{Ptr, DynBase};
use crate::data::Value;
use crate::cast::PtrNonNull;

pub trait ValueToRust<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast(value: Value<'a>) -> Result<Self::CastResult, String>;
}

trait ValueToRustImpl<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast_impl(value: Value<'a>) -> Result<Self::CastResult, String>;
}

trait ValueToRustImpl2<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast_impl2(value: Value<'a>) -> Result<Self::CastResult, String>;
}

impl<'a, T: 'a> ValueToRust<'a, T> for () {
    default unsafe fn any_cast(value: Value<'a>) -> Result<T, String> {
        if value.is_null() {
            Err("NullPointerException".to_string())
        } else {
            <() as ValueToRustImpl<T>>::any_cast_impl(value)
        }
    }
}

impl<'a, T: 'a> ValueToRust<'a, Option<T>> for () {
    unsafe fn any_cast(value: Value<'a>) -> Result<Option<T>, String> {
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(<() as ValueToRustImpl<'a, T>>::any_cast_impl(value)?))
        }
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, T> for () {
    default unsafe fn any_cast_impl(value: Value<'a>) -> Result<T, String> {
        <() as ValueToRustImpl2<'a, T>>::any_cast_impl2(value)
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, &'a T> for () {
    unsafe fn any_cast_impl(value: Value<'a>) -> Result<&'a T, String> {
        if value.is_ptr() {
            Ok((value.data.ptr as *mut T as *const T).as_ref().unwrap())
        } else {
            Err("Source value must be pointer/reference".to_string())
        }
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, &'a mut T> for () {
    unsafe fn any_cast_impl(value: Value<'a>) -> Result<&'a mut T, String> {
        if value.is_ptr() {
            Ok((value.data.ptr as *mut T).as_mut().unwrap())
        } else {
            Err("Source value must be pointer/reference".to_string())
        }
    }
}

impl<'a, T: 'a> ValueToRustImpl2<'a, T> for () {
    default unsafe fn any_cast_impl2(value: Value<'a>) -> Result<T, String> {
        Ok(*Box::from_raw(value.data.ptr as *mut T))
    }
}

impl<'a, T: 'a + Copy> ValueToRustImpl2<'a, T> for () {
    unsafe fn any_cast_impl2(value: Value<'a>) -> Result<T, String> {
        Ok(*(value.data.ptr as *mut T).as_ref().unwrap())
    }
}
