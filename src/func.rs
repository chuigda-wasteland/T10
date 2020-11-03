#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::any::TypeId;
use std::marker::PhantomData;
use std::sync::atomic::Ordering::SeqCst;

use crate::pylon::{Ptr, GcInfo, DynBase, VMPtrToRust};
use crate::tyck::{StaticBase, TypeCheckInfo};

pub enum Storage {
    VMOwned,
    SharedWithHost,
    MutSharedWithHost
}

pub enum RustArgLifetime {
    Move, Copy, Share, MutShare
}

pub trait RustCallable<'a> {
    fn is_unsafe(&self) -> bool;

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustArgLifetime)>;

    fn return_value_spec(&self) -> (TypeCheckInfo, RustArgLifetime);

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Result<Ptr<'a>, String>;

    fn call(&self,
            args: &'a [Ptr<'a>],
            ret_tyck_info: Option<TypeCheckInfo>) -> Result<Ptr<'a>, String> {
        let param_specs = self.param_specs();

        if args.len() != param_specs.len() {
            return Err(format!("expected {} args, got {}", param_specs.len(), args.len()));
        }

        for ((tyck_info, lifetime), (n, ptr)) in
            param_specs.into_iter().zip(args.iter().enumerate())
        {
            if !ptr.data.dyn_type_check(&tyck_info) {
                return Err(format!("type check failed for {}th argument", n))
            }
            if !ptr.lifetime_check(&lifetime) {
                return Err(format!("lifetime check failed for {}th argument", n))
            }
        }

        return unsafe { self.call_prechecked(args) }
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

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustArgLifetime)> {
        vec![
            (A::tyck_info(), A::lifetime_info()),
            (B::tyck_info(), B::lifetime_info())
        ]
    }

    fn return_value_spec(&self) -> (TypeCheckInfo, RustArgLifetime) {
        (RET::tyck_info(), RET::lifetime_info())
    }

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Result<Ptr<'a>, String> {
        let ret = self.inner.call((
            <() as VMPtrToRust<'a, A>>::any_cast(args[0].clone())?
            , <() as VMPtrToRust<'a, B>>::any_cast(args[1].clone())?
        ));
        todo!()
    }
}
