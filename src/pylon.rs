#![allow(dead_code)]
#![allow(unused_variables)]

// use std::marker::PhantomData;
use std::sync::atomic::AtomicPtr;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::SeqCst;

use crate::tyck::{StaticBase, TypeCheckInfo};

pub enum GcInfo {
    OnVMStack         = 0,
    OnVMHeap          = 1,
    SharedWithHost    = 2,
    MutSharedWithHost = 3,
    MovedToHost       = 4
}

pub struct Ptr {
    pub gc_info: AtomicPtr<u8>,
    pub data: *mut dyn DynBase
}

pub struct PtrNonNull {
    pub gc_info: AtomicPtr<u8>,
    pub data: NonNull<dyn DynBase>
}

impl PtrNonNull {
    pub fn from_ptr(ptr: Ptr) -> Option<PtrNonNull> {
        Some(Self {
            gc_info: ptr.gc_info,
            data: NonNull::new(ptr.data)?
        })
    }
}

pub trait DynBase {
    fn static_type_id(&self) -> std::any::TypeId;

    fn static_type_name(&self) -> &'static str;

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool;

    fn dyn_type_check_info(&self) -> TypeCheckInfo;

    unsafe fn as_ptr(&self) -> *mut () {
        self as *const Self as *mut Self as *mut ()
    }
}

// impl !DynBase for &T {}
// impl !DynBase for &mut T {}

impl<T: 'static> DynBase for T {
    fn static_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn dyn_type_check_info(&self) -> TypeCheckInfo {
        <T as StaticBase>::type_check_info()
    }
}

pub trait VMPtrToRust<T> {
    type CastResult = T;

    unsafe fn any_cast(ptr: Ptr) -> Result<Self::CastResult, String>;
}

pub trait VMPtrToRustImpl<T> {
    type CastResult = T;

    unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<Self::CastResult, String>;
}

pub trait VMPtrToRustImpl2<T> {
    type CastResult = T;

    unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<Self::CastResult, String>;
}

impl<T> VMPtrToRust<T> for () {
    default unsafe fn any_cast(ptr: Ptr) -> Result<T, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Err("nullptr exception".to_string())
        }, |ptr| {
            <() as VMPtrToRustImpl<T>>::any_cast_impl(ptr)
        })
    }
}

impl<T> VMPtrToRust<Option<T>> for () {
    unsafe fn any_cast(ptr: Ptr) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as VMPtrToRustImpl<T>>::any_cast_impl(ptr)?))
        })
    }
}

impl<T> VMPtrToRustImpl<T> for () {
    default unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<T, String> {
        <() as VMPtrToRustImpl2<T>>::any_cast_impl2(ptr)
    }
}

impl<'a, T> VMPtrToRustImpl<&'a T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<&'a T, String> {
        unimplemented!()
    }
}

impl<'a, T> VMPtrToRustImpl<&'a mut T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull) -> Result<&'a mut T, String> {
        unimplemented!()
    }
}

impl<T> VMPtrToRustImpl2<T> for () {
    default unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        let r = ptr.gc_info.load(SeqCst).as_mut().unwrap();
        /*
        match *r {
            // should match lifetimes here
        }
        */
        *r = GcInfo::MovedToHost as u8;
        // let data = Box::from_raw(ptr.data.as_mut());
        // let mut ret = MaybeUninit::<T>::uninit();
        // data.inner_move(&mut ret as *mut MaybeUninit<T> as *mut ());
        // Ok(ret.assume_init())

        todo!()
    }
}

impl<T: Copy> VMPtrToRustImpl2<T> for () {
    unsafe fn any_cast_impl2(_ptr: PtrNonNull) -> Result<T, String> {
        // Ok((ptr.data.as_ref().inner_ref() as *const T).as_ref().unwrap().clone())
        todo!()
    }
}
