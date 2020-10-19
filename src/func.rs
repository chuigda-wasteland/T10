use std::any::TypeId;
use std::marker::PhantomData;
use std::sync::atomic::Ordering::SeqCst;

use crate::pylon::{Ptr, GcInfo};

pub enum Storage {
    VMOwned,
    SharedWithHost,
    MutSharedWithHost
}

pub enum RustArgStrategy {
    Move, Copy, Share, MutShare
}

pub trait RustCallable<'a> {
    fn is_unsafe(&self) -> bool;

    fn param_specs(&self) -> &'static [(TypeId, RustArgStrategy)];

    unsafe fn call_prechecked(&self, args: &[Ptr<'a>]) -> Ptr<'a>;

    fn call(&self, args: &[Ptr<'a>]) -> Result<Ptr<'a>, &'static str> {
        let param_spec = self.param_specs();
        if param_spec.len() != args.len() {
            return Err("incorrect argument count")
        }

        for (arg, (param_type, param_strategy)) in args.iter().zip(param_spec.iter()) {
            if arg.static_type_id() != *param_type {
                return Err("incorrect argument type")
            }

            let _: PhantomData<i32> = match (unsafe { arg.gc_info.load(SeqCst).as_ref().unwrap() },
                                             param_strategy) {
                (GcInfo::OnVMStack, RustArgStrategy::Share) => PhantomData::default(),
                (GcInfo::OnVMStack, RustArgStrategy::MutShare) => PhantomData::default(),
                (GcInfo::OnVMHeap, _) => PhantomData::default(),
                (GcInfo::SharedWithHost, RustArgStrategy::Share) => PhantomData::default(),
                (GcInfo::MutSharedWithHost, RustArgStrategy::Share) => PhantomData::default(),
                (GcInfo::MutSharedWithHost, RustArgStrategy::MutShare) => PhantomData::default(),
                _ => return Err("other lifetime error")
            };
        }

        unsafe {
            Ok(self.call_prechecked(args))
        }
    }
}
