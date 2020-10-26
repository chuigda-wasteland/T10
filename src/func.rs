use std::any::TypeId;
use std::marker::PhantomData;
use std::sync::atomic::Ordering::SeqCst;

use crate::pylon::{Ptr, GcInfo};
use crate::tyck::TypeCheckInfo;

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

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustArgStrategy)>;

    fn return_value_spec(&self) -> (TypeCheckInfo, RustArgStrategy);

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Ptr<'a>;

    fn call(&self,
            _args: &'a [Ptr<'a>],
            _ret_tyck_info: Option<TypeCheckInfo>) -> Result<Ptr<'a>, String> {
        unimplemented!()
    }
}

pub struct RustCallBind2<A, B, RET, FN>
    where A: 'static,
          B: 'static,
          RET: 'static,
          FN: 'static + Fn(A, B) -> RET {
    inner: FN,
    _phantom: PhantomData<(A, B, RET)>
}

impl<'a, A, B, RET, FN> RustCallable<'a> for RustCallBind2<A, B, RET, FN>
    where A: 'static,
          B: 'static,
          RET: 'static,
          FN: 'static + Fn(A, B) -> RET {
    fn is_unsafe(&self) -> bool {
        false
    }

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustArgStrategy)> {
        unimplemented!()
    }

    fn return_value_spec(&self) -> (TypeCheckInfo, RustArgStrategy) {
        unimplemented!()
    }

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Ptr<'a> {
        unimplemented!()
    }
}
