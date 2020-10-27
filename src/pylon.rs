#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::atomic::AtomicPtr;
use std::any::TypeId;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::tyck::{StaticBase, TypeCheckInfo};

pub enum GcInfo {
    OnVMStack         = 0,
    OnVMHeap          = 1,
    SharedWithHost    = 2,
    MutSharedWithHost = 3
}

pub struct Ptr<'a> {
    pub gc_info: AtomicPtr<GcInfo>,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>
}

pub struct PtrNonNull<'a> {
    pub gc_info: AtomicPtr<GcInfo>,
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

    fn maybe_type_name(&self) -> Option<&'static str>;

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool;
}

pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    pub inner: Ta,
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

    fn maybe_type_name(&self) -> Option<&'static str> {
        None
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool {
        <Wrapper<'a, Ta, Ts> as StaticBase>::type_check(tyck_info)
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

impl<'a, T: 'static> StaticBase for &'a T {
    fn type_check(_tyck_info: &TypeCheckInfo) -> bool {
        unimplemented!("This is not implemented since it will not be used")
    }

    fn type_check_info() -> TypeCheckInfo {
        <() as TypeCheckExtractor<T>>::type_check_info()
    }
}

impl<'a, T: 'static> StaticBase for &'a mut T {
    fn type_check(type_check_info: &TypeCheckInfo) -> bool {
        unimplemented!()
    }

    fn type_check_info() -> TypeCheckInfo {
        unimplemented!()
    }
}

pub trait TypeCheckExtractor<T: 'static> {
    fn type_check_info() -> TypeCheckInfo;
}

impl<T: 'static> TypeCheckExtractor<T> for () {
    default fn type_check_info() -> TypeCheckInfo {
        <() as TypeCheckExtractor<StaticWrapper<T>>>::type_check_info()
    }
}

impl<T: 'static + StaticBase> TypeCheckExtractor<T> for () {
    fn type_check_info() -> TypeCheckInfo {
        T::type_check_info()
    }
}

pub trait VMTypeToRustStatic<'a, T> {
    type CastResult = T;

    fn any_cast(ptr: Ptr<'a>, tyck_info: &TypeCheckInfo) -> Result<Self::CastResult, String>;

    unsafe fn any_cast_no_tyck(ptr: Ptr<'a>) -> Result<Self::CastResult, String>;
}

pub trait VMTypeToRustStatic2<'a, T> {
    type CastResult = T;

    fn any_cast2(ptr: PtrNonNull<'a>, tyck_info: &TypeCheckInfo)
        -> Result<Self::CastResult, String>;

    unsafe fn any_cast2_no_tyck(ptr: PtrNonNull<'a>)
        -> Result<Self::CastResult, String>;
}

impl<'a, T: 'static> VMTypeToRustStatic<'a, T> for () {
    default fn any_cast(ptr: Ptr<'a>, tyck_info: &TypeCheckInfo) -> Result<T, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Err("nullptr exception".to_string())
        }, |ptr| {
            <() as VMTypeToRustStatic2<T>>::any_cast2(ptr, tyck_info)
        })
    }

    default unsafe fn any_cast_no_tyck(ptr: Ptr<'a>) -> Result<T, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Err("nullptr exception".to_string())
        }, |ptr| {
            <() as VMTypeToRustStatic2<T>>::any_cast2_no_tyck(ptr)
        })
    }
}

impl<'a, T: 'static> VMTypeToRustStatic<'a, Option<T>> for () {
    fn any_cast(ptr: Ptr<'a>, tyck_info: &TypeCheckInfo) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as VMTypeToRustStatic2<T>>::any_cast2(ptr, tyck_info)?))
        })
    }

    unsafe fn any_cast_no_tyck(ptr: Ptr<'a>) -> Result<Option<T>, String> {
        PtrNonNull::from_ptr(ptr).map_or_else(|| {
            Ok(None)
        }, |ptr| {
            Ok(Some(<() as VMTypeToRustStatic2<T>>::any_cast2_no_tyck(ptr)?))
        })
    }
}

impl<'a, T: 'static> VMTypeToRustStatic2<'a, T> for () {
    default fn any_cast2(ptr: PtrNonNull<'a>, tyck_info: &TypeCheckInfo) -> Result<T, String> {
        unimplemented!()
    }

    default unsafe fn any_cast2_no_tyck(ptr: PtrNonNull<'a>) -> Result<T, String> {
        unimplemented!()
    }
}

impl<'a, T: 'static + Copy> VMTypeToRustStatic2<'a, T> for () {
    fn any_cast2(ptr: PtrNonNull<'a>, tyck_info: &TypeCheckInfo) -> Result<T, String> {
        unimplemented!()
    }

    unsafe fn any_cast2_no_tyck(ptr: PtrNonNull<'a>) -> Result<T, String> {
        unimplemented!()
    }
}