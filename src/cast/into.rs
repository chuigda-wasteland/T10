use std::sync::atomic::Ordering::SeqCst;

use crate::data::{Ptr, GcInfo};
use crate::cast::{PtrNonNull, RustLifetime, lifetime_check};

pub trait VMPtrToRust<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast(ptr: Ptr<'a>) -> Result<Self::CastResult, String>;
}

trait VMPtrToRustImpl<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<Self::CastResult, String>;
}

trait VMPtrToRustImpl2<'a, T: 'a> {
    type CastResult = T;

    unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<Self::CastResult, String>;
}

impl<'a, T: 'a> VMPtrToRust<'a, T> for () {
    default unsafe fn any_cast(ptr: Ptr<'a>) -> Result<T, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Err("nullptr exception".to_string())
        }, |ptr| {
            <() as VMPtrToRustImpl<'a, T>>::any_cast_impl(ptr)
        })
    }
}

impl<'a, T: 'a> VMPtrToRust<'a, Option<T>> for () {
    unsafe fn any_cast(ptr: Ptr<'a>) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as VMPtrToRustImpl<'a, T>>::any_cast_impl(ptr)?))
        })
    }
}

impl<'a, T: 'a> VMPtrToRustImpl<'a, T> for () {
    default unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<T, String> {
        <() as VMPtrToRustImpl2<'a, T>>::any_cast_impl2(ptr)
    }
}

impl<'a, T: 'a> VMPtrToRustImpl<'a, &'a T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustLifetime::Share)?;
        Ok((ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}

impl<'a, T: 'a> VMPtrToRustImpl<'a, &'a mut T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a mut T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustLifetime::MutShare)?;
        Ok((ptr.data.as_ptr() as *mut T).as_mut().unwrap())
    }
}

impl<'a, T: 'a> VMPtrToRustImpl2<'a, T> for () {
    default unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustLifetime::Move)?;
        Ok(*Box::from_raw(ptr.data.as_ptr() as *mut T))
    }
}

impl<'a, T: 'a + Copy> VMPtrToRustImpl2<'a, T> for () {
    unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustLifetime::Copy)?;
        Ok(*(ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}
