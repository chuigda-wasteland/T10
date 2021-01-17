//! `func` 模块中定义了与 FFI 调用函数相关的接口

use std::marker::PhantomData;

use crate::cast::from_value::FromValue;
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
    f: F,
    _phantom: PhantomData<(&'a (), A, B, RET)>
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
        let mut arg1_guard = <Void as FromValue<A>>::lifetime_check(arg1)?;
        let mut arg2_guard = <Void as FromValue<B>>::lifetime_check(arg2)?;

        let ret = (self.f)(
            <Void as FromValue<A>>::from_value(arg1),
            <Void as FromValue<B>>::from_value(arg2)
        );
        arg1_guard.finish();
        arg2_guard.finish();

        <Void as IntoValue<RET>>::into_value(ret)
    }
}
