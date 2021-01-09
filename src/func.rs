//! `func` 模块中定义了与 FFI 调用函数相关的接口

use std::marker::PhantomData;

use crate::cast::from_value::FromValue;
use crate::cast::into_value::IntoValueL1;
use crate::data::Value;
use crate::error::TError;
use crate::tyck::{FFIAction, TypeCheckInfo};
use crate::tyck::base::StaticBase;
use crate::void::Void;

pub trait RustCallable<'a> {
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction)>;
    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction);
    unsafe fn call_prechecked(&self, args: &'a [Value<'a>]) -> Result<Value<'a>, TError>;
}

pub struct RustFunction<'a, F, A, B, RET>
    where F: 'static + Fn(A, B) -> RET + Send + Sync,
          A: 'a,
          B: 'a,
          RET: 'a,
          Void: FromValue<'a, A>,
          Void: FromValue<'a, B>,
          Void: IntoValueL1<'a, RET>
{
    f: F,
    _phantom: PhantomData<(&'a (), A, B, RET)>
}

impl<'a, F, A, B, RET> RustCallable<'a> for RustFunction<'a, F, A, B, RET>
    where F: 'static + Fn(A, B) -> RET + Send + Sync,
          A: 'a,
          B: 'a,
          RET: 'a,
          Void: FromValue<'a, A>,
          Void: FromValue<'a, B>,
          Void: IntoValueL1<'a, RET> {
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction)> {
        todo!()
    }

    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction) {
        todo!()
    }

    unsafe fn call_prechecked(&self, args: &'a [Value<'a>]) -> Result<Value<'a>, TError> {
        todo!()
    }
}
