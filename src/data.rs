use std::sync::atomic::AtomicU8;
use std::marker::PhantomData;

use crate::tyck::{StaticBase, TypeCheckInfo};
use std::sync::atomic::Ordering::SeqCst;

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
    pub gc_info: *mut AtomicU8,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Ptr<'a> {
    pub fn gc_info(&self) -> GcInfo {
        unsafe {
            if let Some(info) = self.gc_info.as_ref() {
                GcInfo::from_u8(info.load(SeqCst))
            } else {
                GcInfo::Dropped
            }
        }
    }
}

impl<'a> Clone for Ptr<'a> {
    fn clone(&self) -> Self {
        Self {
            gc_info: self.gc_info,
            data: self.data,
            _phantom: self._phantom,
        }
    }
}

unsafe impl<'a> Send for Ptr<'a> {}
unsafe impl<'a> Sync for Ptr<'a> {}
