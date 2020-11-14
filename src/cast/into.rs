use crate::data::Ptr;
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
            () as ValueToRustImpl<>
        }
    }
}

impl<'a, T: 'a> ValueToRust<'a, Option<T>> for () {
    unsafe fn any_cast(ptr: Ptr<'a>) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as ValueToRustImpl<'a, T>>::any_cast_impl(ptr)?))
        })
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, T> for () {
    default unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<T, String> {
        <() as ValueToRustImpl2<'a, T>>::any_cast_impl2(ptr)
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, &'a T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a T, String> {
        Ok((ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}

impl<'a, T: 'a> ValueToRustImpl<'a, &'a mut T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a mut T, String> {
        Ok((ptr.data.as_ptr() as *mut T).as_mut().unwrap())
    }
}

impl<'a, T: 'a> ValueToRustImpl2<'a, T> for () {
    default unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        Ok(*Box::from_raw(ptr.data.as_ptr() as *mut T))
    }
}

impl<'a, T: 'a + Copy> ValueToRustImpl2<'a, T> for () {
    unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        Ok(*(ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}
