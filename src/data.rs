use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::SeqCst;
use std::marker::PhantomData;

use crate::tyck::{StaticBase, TypeCheckInfo};

pub trait DynBase {
    fn dyn_type_id(&self) -> std::any::TypeId;

    fn dyn_type_name(&self) -> &'static str;

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> bool;

    fn dyn_tyck_info(&self) -> TypeCheckInfo;

    unsafe fn as_ptr(&self) -> *mut () {
        self as *const Self as *mut Self as *mut ()
    }
}

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
