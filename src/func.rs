use crate::data::Value;
use crate::error::TError;
use crate::tyck::{TypeCheckInfo, FFIAction};

pub trait RustCallable<'a> {
    fn param_specs(&self) -> Vec<(TypeCheckInfo, FFIAction)>;
    fn return_value_spec(&self) -> (TypeCheckInfo, FFIAction);
    unsafe fn call_prechecked(&self, args: &'a [Value<'a>]) -> Result<Value<'a>, TError>;
}
