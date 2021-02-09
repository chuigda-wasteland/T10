//! `func` 模块中定义了与 FFI 调用函数相关的接口

use std::marker::PhantomData;

use crate::cast::from_value::{FromValue, GcInfoGuard};
use crate::cast::into_value::IntoValue;
use crate::data::Value;
use crate::error::TError;
use crate::tyck::{FFIAction, TypeCheckInfo};
use crate::tyck::fusion::{ExceptionSpec, Fusion, FusionRV, Nullable};
use crate::void::Void;

pub trait RustCallable<'a> {
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction, Nullable)>;
    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction, ExceptionSpec);
    unsafe fn call_prechecked(&self, args: &'a [Value]) -> Result<Value, TError>;
}

pub struct RustFunction<'a, F, A, B, RET>
    where F: 'static + Fn(A, B) -> RET + Send + Sync,
          A: 'a,
          B: 'a,
          RET: 'a,
          Void: FromValue<'a, A> + Fusion<A>,
          Void: FromValue<'a, B> + Fusion<B>,
          Void: IntoValue<'a, RET> + FusionRV<RET>
{
    pub f: F,
    pub _phantom: PhantomData<(&'a (), A, B, RET)>
}

impl<'a, F, A, B, RET> RustCallable<'a> for RustFunction<'a, F, A, B, RET>
    where F: 'static + Fn(A, B) -> RET + Send + Sync,
          A: 'a,
          B: 'a,
          RET: 'a,
          Void: FromValue<'a, A> + Fusion<A>,
          Void: FromValue<'a, B> + Fusion<B>,
          Void: IntoValue<'a, RET> + FusionRV<RET>
{
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction, Nullable)> {
        vec![
            (<Void as Fusion<A>>::fusion_tyck_info(),
             <Void as Fusion<A>>::fusion_ffi_action(),
             <Void as Fusion<A>>::nullable()),
            (<Void as Fusion<B>>::fusion_tyck_info(),
             <Void as Fusion<B>>::fusion_ffi_action(),
             <Void as Fusion<A>>::nullable()),
        ]
    }

    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction, ExceptionSpec) {
        (<Void as FusionRV<RET>>::tyck_info_rv(),
         <Void as FusionRV<RET>>::ffi_action_rv(),
         <Void as FusionRV<RET>>::exception())
    }

    unsafe fn call_prechecked(&self, args: &'a [Value]) -> Result<Value, TError> {
        debug_assert_eq!(args.len(), 2);
        let arg1 = args.get_unchecked(0);
        let arg2 = args.get_unchecked(1);
        let mut arg1_guard: GcInfoGuard = <Void as FromValue<A>>::lifetime_check(arg1).unwrap();
        let mut arg2_guard: GcInfoGuard = <Void as FromValue<B>>::lifetime_check(arg2).unwrap();

        let ret = (self.f)(
            <Void as FromValue<A>>::from_value(arg1),
            <Void as FromValue<B>>::from_value(arg2)
        );
        arg1_guard.finish();
        arg2_guard.finish();

        <Void as IntoValue<RET>>::into_value(ret)
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use std::marker::PhantomData;
    use test::Bencher;

    use crate::data::{StaticWrapper, DynBase};
    use crate::func::{Value, RustFunction, RustCallable};

    struct S(i32);

    fn bar(x: &mut S, y: &S) -> i64 {
        x.0 = y.0;
        x.0 as i64
    }

    #[test] fn test_simple_call() {
        let s1 = Box::leak(Box::new(StaticWrapper::owned(S(0)))) as *mut dyn DynBase;
        let s2 = Box::leak(Box::new(StaticWrapper::owned(S(4)))) as *mut dyn DynBase;
        let v1 = Value::from(s1);
        let v2 = Value::from(s2);
        let f = RustFunction { f: bar, _phantom: PhantomData::default() };
        unsafe {
            f.call_prechecked(&[v1, v2]).unwrap();
        }
    }

    fn baz(x: i64, y: i64) -> i64 {
        x + y
    }

    #[test] fn test_simple_call2() {
        let v1 = Value::from(14i64);
        let v2 = Value::from(40i64);
        let f = RustFunction { f: baz, _phantom: PhantomData::default() };
        unsafe {
            assert_eq!(f.call_prechecked(&[v1, v2]).unwrap().value_typed_data.inner.int, 54);
        }
    }

    #[bench] fn bench_simple_call2(b: &mut Bencher) {
        let f = RustFunction { f: baz, _phantom: PhantomData::default() };
        b.iter(|| {
            for i in 0..1000i64 {
                for j in 0..1000i64 {
                    let v1 = Value::from(i);
                    let v2 = Value::from(j);
                    unsafe {
                        let _ = f.call_prechecked(&[v1, v2]);
                    }
                }
            }
        })
    }

}

