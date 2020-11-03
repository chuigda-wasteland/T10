#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;
use std::sync::atomic::AtomicPtr;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::SeqCst;

use crate::tyck::{StaticBase, TypeCheckInfo};
use crate::func::RustArgLifetime;

pub enum GcInfo {
    OnVMStack         = 0,
    OnVMHeap          = 1,
    SharedWithHost    = 2,
    MutSharedWithHost = 3,
    MovedToHost       = 4
}

impl GcInfo {
    pub fn from_u8(src: u8) -> GcInfo {
        match src {
            0 => GcInfo::OnVMStack,
            1 => GcInfo::OnVMHeap,
            2 => GcInfo::SharedWithHost,
            3 => GcInfo::MutSharedWithHost,
            4 => GcInfo::MovedToHost,
            _ => unreachable!()
        }
    }
}

pub struct Ptr<'a> {
    pub gc_info: AtomicPtr<u8>,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>
}

impl<'a> Clone for Ptr<'a> {
    fn clone(&self) -> Self {
        Self {
            gc_info: AtomicPtr::new(self.gc_info.load(SeqCst)),
            data: self.data,
            _phantom: self._phantom
        }
    }
}

impl<'a> Ptr<'a> {
    pub fn lifetime_check(&self, lifetime: &RustArgLifetime) -> bool {
        match (GcInfo::from_u8(unsafe{*self.gc_info.load(SeqCst)}), lifetime) {
            (GcInfo::OnVMStack, RustArgLifetime::Share) => true,
            (GcInfo::OnVMStack, RustArgLifetime::MutShare) => true,
            (GcInfo::OnVMStack, RustArgLifetime::Copy) => true,
            (GcInfo::OnVMStack, RustArgLifetime::Move) =>
                unimplemented!("items on stack should be Copy"),
            (GcInfo::OnVMHeap, RustArgLifetime::Share) => true,
            (GcInfo::OnVMHeap, RustArgLifetime::MutShare) => true,
            (GcInfo::OnVMHeap, RustArgLifetime::Copy) => true,
            (GcInfo::OnVMHeap, RustArgLifetime::Move) => true,
            (GcInfo::SharedWithHost, RustArgLifetime::Copy) => true,
            (GcInfo::SharedWithHost, RustArgLifetime::Share) => true,
            (GcInfo::MutSharedWithHost, RustArgLifetime::Copy) => true,
            _ => false
        }
    }
}

pub struct PtrNonNull<'a> {
    pub gc_info: AtomicPtr<u8>,
    pub data: NonNull<dyn DynBase>,
    _phantom: PhantomData<&'a ()>
}

impl<'a> PtrNonNull<'a> {
    pub fn from_ptr(ptr: Ptr<'a>) -> Option<PtrNonNull<'a>> {
        Some(Self {
            gc_info: ptr.gc_info,
            data: NonNull::new(ptr.data)?,
            _phantom: PhantomData::default()
        })
    }
}

pub trait DynBase {
    fn dyn_type_id(&self) -> std::any::TypeId;

    fn dyn_type_name(&self) -> &'static str;

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool;

    fn dyn_tyck_info(&self) -> TypeCheckInfo;

    unsafe fn as_ptr(&self) -> *mut () {
        self as *const Self as *mut Self as *mut ()
    }
}

// impl !DynBase for &T {}
// impl !DynBase for &mut T {}

impl<T: 'static> DynBase for T {
    fn dyn_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn dyn_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn dyn_tyck_info(&self) -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
    }
}

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
        unimplemented!()
    }
}

impl<'a, T: 'a> VMPtrToRustImpl<'a, &'a mut T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a mut T, String> {
        unimplemented!()
    }
}

impl<'a, T: 'a> VMPtrToRustImpl2<'a, T> for () {
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

impl<'a, T: 'a + Copy> VMPtrToRustImpl2<'a, T> for () {
    unsafe fn any_cast_impl2(_ptr: PtrNonNull) -> Result<T, String> {
        // Ok((ptr.data.as_ref().inner_ref() as *const T).as_ref().unwrap().clone())
        todo!()
    }
}
