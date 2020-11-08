pub mod from;
pub mod into;

use std::sync::atomic::AtomicU8;
use std::ptr::NonNull;
use std::marker::PhantomData;

use crate::data::{DynBase, Ptr, GcInfo};
use std::sync::atomic::Ordering::SeqCst;

pub struct PtrNonNull<'a> {
    pub gc_info: *mut AtomicU8,
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

    pub fn gc_info(&self) -> GcInfo {
        unsafe {
            GcInfo::from_u8(self.gc_info.as_ref().unwrap().load(SeqCst))
        }
    }
}

pub enum RustLifetime {
    Move,
    Copy,
    Share,
    MutShare,
}

pub fn lifetime_check(gc_info: &GcInfo, lifetime: &RustLifetime) -> Result<(), String> {
    match (gc_info, lifetime) {
        (GcInfo::OnVMStack, RustLifetime::Share) => Ok(()),
        (GcInfo::OnVMStack, RustLifetime::MutShare) => Ok(()),
        (GcInfo::OnVMStack, RustLifetime::Copy) => Ok(()),
        (GcInfo::OnVMStack, RustLifetime::Move) =>
            unreachable!("items on stack should be Copy"),

        (GcInfo::OnVMHeap, RustLifetime::Share) => Ok(()),
        (GcInfo::OnVMHeap, RustLifetime::MutShare) => Ok(()),
        (GcInfo::OnVMHeap, RustLifetime::Copy) => Ok(()),
        (GcInfo::OnVMHeap, RustLifetime::Move) => Ok(()),

        (GcInfo::SharedWithHost, RustLifetime::Copy) => Ok(()),
        (GcInfo::SharedWithHost, RustLifetime::Share) => Ok(()),
        (GcInfo::SharedWithHost, RustLifetime::Move) =>
            Err("cannot move shared item".to_string()),
        (GcInfo::SharedWithHost, RustLifetime::MutShare) =>
            Err("cannot mutably share an immutably shared item".to_string()),

        (GcInfo::MutSharedWithHost, RustLifetime::Copy) => Ok(()),
        (GcInfo::MutSharedWithHost, RustLifetime::Move) =>
            Err("cannot move shared item".to_string()),
        (GcInfo::MutSharedWithHost, RustLifetime::Share) =>
            Err("cannot immutably share a mutably shared item".to_string()),
        (GcInfo::MutSharedWithHost, RustLifetime::MutShare) =>
            Err("cannot mutably share item twice".to_string()),

        (GcInfo::MovedToHost, _) => Err("operating an moved item".to_string()),
        (GcInfo::Dropped, _) => unreachable!("cannot use a dropped item")
    }
}

