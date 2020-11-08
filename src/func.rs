use std::marker::PhantomData;
use std::sync::atomic::Ordering::SeqCst;

use crate::data::{DynBase, Ptr, GcInfo};
use crate::tyck::{StaticBase, TypeCheckInfo};
use crate::cast::{RustLifetime, lifetime_check};
use crate::cast::into::VMPtrToRust;

pub trait RustCallable<'a> {
    fn is_unsafe(&self) -> bool;

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustLifetime)>;

    fn return_value_spec(&self) -> (TypeCheckInfo, RustLifetime);

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Result<Ptr<'a>, String>;

    fn call(&self,
            args: &'a [Ptr<'a>],
            _ret_tyck_info: Option<TypeCheckInfo>)
        -> Result<Ptr<'a>, String>
    {
        let param_specs = self.param_specs();

        if args.len() != param_specs.len() {
            return Err(format!("expected {} args, got {}", param_specs.len(), args.len()));
        }

        for ((tyck_info, lifetime), (n, ptr))
            in param_specs.into_iter().zip(args.iter().enumerate())
        {
            if !ptr.data.dyn_type_check(&tyck_info) {
                return Err(format!("type check failed for {}th argument", n));
            }
            lifetime_check(&GcInfo::from_u8(unsafe { ptr.gc_info.as_ref().unwrap().load(SeqCst) }), &lifetime)?;
        }

        return unsafe { self.call_prechecked(args) };
    }
}

pub struct RustCallBind2<A, B, RET, FN>
    where A: 'static,
          B: 'static,
          RET: 'static,
          FN: 'static + Fn(A, B) -> RET
{
    inner: FN,
    _phantom: PhantomData<(A, B, RET)>,
}

impl<'a, A, B, RET, FN> RustCallable<'a> for RustCallBind2<A, B, RET, FN>
    where A: 'static,
          B: 'static,
          RET: 'static,
          FN: 'static + Fn(A, B) -> RET
{
    fn is_unsafe(&self) -> bool {
        false
    }

    fn param_specs(&self) -> Vec<(TypeCheckInfo, RustLifetime)> {
        vec![
            (A::tyck_info(), A::lifetime_info()),
            (B::tyck_info(), B::lifetime_info())
        ]
    }

    fn return_value_spec(&self) -> (TypeCheckInfo, RustLifetime) {
        (RET::tyck_info(), RET::lifetime_info())
    }

    unsafe fn call_prechecked(&self, args: &'a [Ptr<'a>]) -> Result<Ptr<'a>, String> {
        let _ret: RET = self.inner.call((
            <() as VMPtrToRust<'a, A>>::any_cast(args[0].clone())?
            , <() as VMPtrToRust<'a, B>>::any_cast(args[1].clone())?
        ));
        todo!()
    }
}
