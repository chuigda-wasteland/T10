#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::SeqCst;

use crate::func::RustArgLifetime;
use crate::tyck::{StaticBase, TypeCheckInfo};

pub enum GcInfo {
    OnVMStack = 0,
    OnVMHeap = 1,
    SharedWithHost = 2,
    MutSharedWithHost = 3,
    MovedToHost = 4,
    Dropped = 5
}

impl GcInfo {
    pub fn from_u8(src: u8) -> GcInfo {
        match src {
            0 => GcInfo::OnVMStack,
            1 => GcInfo::OnVMHeap,
            2 => GcInfo::SharedWithHost,
            3 => GcInfo::MutSharedWithHost,
            4 => GcInfo::MovedToHost,
            5 => GcInfo::Dropped,
            _ => unreachable!()
        }
    }
}

pub fn lifetime_check(gc_info: &GcInfo, lifetime: &RustArgLifetime) -> Result<(), String> {
    match (gc_info, lifetime) {
        (GcInfo::OnVMStack, RustArgLifetime::Share) => Ok(()),
        (GcInfo::OnVMStack, RustArgLifetime::MutShare) => Ok(()),
        (GcInfo::OnVMStack, RustArgLifetime::Copy) => Ok(()),
        (GcInfo::OnVMStack, RustArgLifetime::Move) =>
            unreachable!("items on stack should be Copy"),

        (GcInfo::OnVMHeap, RustArgLifetime::Share) => Ok(()),
        (GcInfo::OnVMHeap, RustArgLifetime::MutShare) => Ok(()),
        (GcInfo::OnVMHeap, RustArgLifetime::Copy) => Ok(()),
        (GcInfo::OnVMHeap, RustArgLifetime::Move) => Ok(()),

        (GcInfo::SharedWithHost, RustArgLifetime::Copy) => Ok(()),
        (GcInfo::SharedWithHost, RustArgLifetime::Share) => Ok(()),
        (GcInfo::SharedWithHost, RustArgLifetime::Move) =>
            Err("cannot move shared item".to_string()),
        (GcInfo::SharedWithHost, RustArgLifetime::MutShare) =>
            Err("cannot mutably share an immutably shared item".to_string()),

        (GcInfo::MutSharedWithHost, RustArgLifetime::Copy) => Ok(()),
        (GcInfo::MutSharedWithHost, RustArgLifetime::Move) =>
            Err("cannot move shared item".to_string()),
        (GcInfo::MutSharedWithHost, RustArgLifetime::Share) =>
            Err("cannot immutably share a mutably shared item".to_string()),
        (GcInfo::MutSharedWithHost, RustArgLifetime::MutShare) =>
            Err("cannot mutably share item twice".to_string()),

        (GcInfo::MovedToHost, _) => Err("operating an moved item".to_string()),
        (GcInfo::Dropped, _) => unreachable!("cannot use a dropped item")
    }
}

pub struct Ptr<'a> {
    pub gc_info: AtomicPtr<u8>,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Clone for Ptr<'a> {
    fn clone(&self) -> Self {
        Self {
            gc_info: AtomicPtr::new(self.gc_info.load(SeqCst)),
            data: self.data,
            _phantom: self._phantom,
        }
    }
}

pub struct PtrNonNull<'a> {
    pub gc_info: AtomicPtr<u8>,
    pub data: NonNull<dyn DynBase>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> PtrNonNull<'a> {
    pub fn from_ptr(ptr: Ptr<'a>) -> Option<PtrNonNull<'a>> {
        Some(Self {
            gc_info: ptr.gc_info,
            data: NonNull::new(ptr.data)?,
            _phantom: PhantomData::default(),
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

pub trait VMPtrFromRust<'a, T: 'a> {
    unsafe fn from_any(t: T) -> Result<Ptr<'a>, String>;
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
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustArgLifetime::Share)?;
        Ok((ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}

impl<'a, T: 'a> VMPtrToRustImpl<'a, &'a mut T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<&'a mut T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustArgLifetime::MutShare)?;
        Ok((ptr.data.as_ptr() as *mut T).as_mut().unwrap())
    }
}

impl<'a, T: 'a> VMPtrToRustImpl2<'a, T> for () {
    default unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustArgLifetime::Move)?;
        Ok(*Box::from_raw(ptr.data.as_ptr() as *mut T))
    }
}

impl<'a, T: 'a + Copy> VMPtrToRustImpl2<'a, T> for () {
    unsafe fn any_cast_impl2(ptr: PtrNonNull) -> Result<T, String> {
        let r = GcInfo::from_u8(*ptr.gc_info.load(SeqCst));
        lifetime_check(&r, &RustArgLifetime::Copy)?;
        Ok(*(ptr.data.as_ptr() as *mut T).as_ref().unwrap())
    }
}
