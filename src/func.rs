//! `func` 模块中定义了与 FFI 调用函数相关的接口

use std::marker::PhantomData;

use crate::cast::from_value::FromValue;
use crate::cast::into_value::IntoValueL1;
use crate::data::Value;
use crate::error::TError;
use crate::tyck::{FFIAction, TypeCheckInfo};
use crate::tyck::fusion::{ExceptionSpec, Fusion, FusionRV, Nullable};
use crate::void::Void;

pub trait RustCallable<'a> {
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction, Nullable)>;
    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction, ExceptionSpec);
    unsafe fn call_prechecked(&self, args: &'a [Value<'a>]) -> Result<Value<'a>, TError>;
}

pub struct RustFunction<'a, F, A, B, RET>
    where F: 'static + Fn(A, B) -> RET + Send + Sync,
          A: 'a,
          B: 'a,
          RET: 'a,
          Void: FromValue<'a, A> + Fusion<A>,
          Void: FromValue<'a, B> + Fusion<B>,
          Void: IntoValueL1<'a, RET> + FusionRV<RET>
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
          Void: IntoValueL1<'a, RET> + FusionRV<RET> {
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

    unsafe fn call_prechecked(&self, args: &'a [Value<'a>]) -> Result<Value<'a>, TError> {
        todo!()
    }
}
