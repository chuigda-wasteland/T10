#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::atomic::AtomicPtr;
use std::any::TypeId;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::cell::Cell;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering::SeqCst;

use crate::tyck::{StaticBase, TypeCheckInfo};

pub enum GcInfo {
    OnVMStack         = 0,
    OnVMHeap          = 1,
    SharedWithHost    = 2,
    MutSharedWithHost = 3,
    MovedToHost       = 4
}

pub struct Ptr<'a> {
    pub gc_info: AtomicPtr<u8>,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>
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
            _phantom: ptr._phantom
        })
    }
}

pub trait DynBase {
    fn static_type_id(&self) -> TypeId;

    fn static_type_name(&self) -> &'static str;

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool;

    unsafe fn inner_ref(&self) -> *mut ();

    unsafe fn inner_move(&self, maybe_uninit: *mut ());
}

impl<'a, T: 'static> DynBase for &'a T {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool {
        <() as TypeCheckExtractor<T>>::type_check(tyck_info)
    }

    unsafe fn inner_ref(&self) -> *mut () {
        unimplemented!()
    }

    unsafe fn inner_move(&self, maybe_uninit: *mut ()) {
        unimplemented!()
    }
}

pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    pub inner: Cell<MaybeUninit<Ta>>,
    _phantom: PhantomData<&'a Ts>
}

pub type StaticWrapper<T> = Wrapper<'static, T, T>;

impl<'a, Ta: 'a, Ts: 'static> DynBase for Wrapper<'a, Ta, Ts> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Ts>()
    }

    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Ts>()
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool {
        <Self as StaticBase>::type_check(tyck_info)
    }

    unsafe fn inner_ref(&self) -> *mut () {
        unimplemented!()
    }

    unsafe fn inner_move(&self, maybe_uninit: *mut ()) {
        let maybe_uninit = (maybe_uninit as *mut MaybeUninit<Ta>).as_mut().unwrap();
        maybe_uninit.write(self.inner.replace(MaybeUninit::uninit()).assume_init());
    }
}

impl<'a, Ta: 'a, Ts: 'static> StaticBase for Wrapper<'a, Ta, Ts> {
    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::SimpleType(type_id) = tyck_info {
            *type_id == std::any::TypeId::of::<Ts>()
        } else {
            false
        }
    }

    fn type_check_info() -> TypeCheckInfo {
        TypeCheckInfo::SimpleType(std::any::TypeId::of::<Ts>())
    }
}

pub trait TypeCheckExtractor<T: 'static> {
    fn type_check_info() -> TypeCheckInfo;

    fn type_check(tyck_info: &TypeCheckInfo) -> bool;
}

impl<T: 'static> TypeCheckExtractor<T> for () {
    default fn type_check_info() -> TypeCheckInfo {
        <() as TypeCheckExtractor<StaticWrapper<T>>>::type_check_info()
    }

    default fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        <() as TypeCheckExtractor<StaticWrapper<T>>>::type_check(tyck_info)
    }
}

impl<T: 'static + StaticBase> TypeCheckExtractor<T> for () {
    fn type_check_info() -> TypeCheckInfo {
        T::type_check_info()
    }

    fn type_check(tyck_info: &TypeCheckInfo) -> bool {
        T::type_check(tyck_info)
    }
}

pub trait VMPtrToRust<'a, T> {
    type CastResult = T;

    unsafe fn any_cast(ptr: Ptr<'a>) -> Result<Self::CastResult, String>;
}

pub trait VMPtrToRustImpl<'a, T> {
    type CastResult = T;

    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<Self::CastResult, String>;
}

impl<'a, T> VMPtrToRust<'a, T> for () {
    default unsafe fn any_cast(ptr: Ptr<'a>) -> Result<T, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Err("nullptr exception".to_string())
        }, |ptr| {
            <() as VMPtrToRustImpl<T>>::any_cast_impl(ptr)
        })
    }
}

impl<'a, T> VMPtrToRust<'a, Option<T>> for () {
    unsafe fn any_cast(ptr: Ptr<'a>) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as VMPtrToRustImpl<T>>::any_cast_impl(ptr)?))
        })
    }
}

impl<'a, T> VMPtrToRustImpl<'a, T> for () {
    default unsafe fn any_cast_impl(mut ptr: PtrNonNull<'a>) -> Result<T, String> {
        let r = ptr.gc_info.load(SeqCst).as_mut().unwrap();
        /*
        match *r {
            // should match lifetimes here
        }
        */
        *r = GcInfo::MovedToHost as u8;
        let data = Box::from_raw(ptr.data.as_mut());
        let mut ret = MaybeUninit::<T>::uninit();
        data.inner_move(&mut ret as *mut MaybeUninit<T> as *mut ());
        Ok(ret.assume_init())
    }
}

impl<'a, T: Copy> VMPtrToRustImpl<'a, T> for () {
    unsafe fn any_cast_impl(ptr: PtrNonNull<'a>) -> Result<T, String> {
        Ok((ptr.data.as_ref().inner_ref() as *const T).as_ref().unwrap().clone())
    }
}
