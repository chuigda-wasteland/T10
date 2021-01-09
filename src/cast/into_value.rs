//! 从 Rust 对象到 `Value` 的转换

use std::error::Error;

use crate::data::{Value, StaticWrapper, DynBase};
use crate::error::TError;
use crate::void::Void;

/// 在这一层 specialization 中特殊处理 `Result<T, E>`
///
/// 对 `Result<T, E>` 的处理只在从 Rust 函数向 T10 运行时返回一个值的时候进行。
pub trait IntoValue<'a, T> {
     fn into_value(t: T) -> Result<Value<'a>, TError>;
}

/// 在这一层的 specialization 中特殊处理 `&Option<T>` 和 `&mut Option<T>`
///
/// `&Option<T>` 或者 `&mut Option<T>` 会被进一步视为 `Option<&T>` 和 `Option<&mut T>`。
/// 这种特殊处理只会在从 Rust 函数向 T10 运行时返回一个值的时候进行。
pub trait IntoValueNoexcept<'a, T> {
     fn into_value_noexcept(t: T) -> Result<Value<'a>, TError>;
}

/// 在这一层的 specialization 中特殊处理 `Option<T>`
pub trait IntoValueL1<'a, T> {
     fn into_value_l1(t: T) -> Result<Value<'a>, TError>;
}

/// 在这一层的 specialization 中处理 `&T` 和 `&mut T`。
///
/// 从 Rust 环境向 T10 运行时传递的引用会被永久视为共享引用；禁止在 T10 通过 FFI 调用 Rust 函数时，
/// Rust 函数向 T10 返回任何引用。
pub trait IntoValueL2<'a, T> {
     fn into_value_l2(t: T) -> Result<Value<'a>, TError>;
}

/// 在这一层的 specialization 中特殊处理 `i64` 之类的值类型
pub trait IntoValueL3<'a, T> {
     fn into_value_l3(t: T) -> Result<Value<'a>, TError>;
}

impl<'a, T> IntoValue<'a, T> for Void where Void: IntoValueNoexcept<'a, T> {
    #[inline] default fn into_value(t: T) -> Result<Value<'a>, TError> {
        <Void as IntoValueNoexcept<'a, T>>::into_value_noexcept(t)
    }
}

impl<'a, T, E> IntoValue<'a, Result<T, E>> for Void
    where Void: IntoValueNoexcept<'a, T>,
          E: 'static + Error
{
    #[inline] fn into_value(t: Result<T, E>) -> Result<Value<'a>, TError> {
        match t {
            Ok(data) => <Void as IntoValueNoexcept<'a, T>>::into_value_noexcept(data),
            Err(e) => Err(TError::user_exception(e))
        }
    }
}

impl<'a, T> IntoValueNoexcept<'a, T> for Void where Void: IntoValueL1<'a, T> {
    #[inline] default fn into_value_noexcept(t: T) -> Result<Value<'a>, TError> {
        <Void as IntoValueL1<'a, T>>::into_value_l1(t)
    }
}

impl<'a, T> IntoValueNoexcept<'a, &'a Option<T>> for Void where T: 'static {
    #[inline] fn into_value_noexcept(t: &'a Option<T>) -> Result<Value<'a>, TError> {
        let t: Option<&'a T> = t.as_ref();
        <Void as IntoValueL1<Option<&'a T>>>::into_value_l1(t)
    }
}

impl<'a, T> IntoValueNoexcept<'a, &'a mut Option<T>> for Void where T: 'static {
    #[inline] fn into_value_noexcept(t: &'a mut Option<T>) -> Result<Value<'a>, TError> {
        let t: Option<&'a mut T> = t.as_mut();
        <Void as IntoValueL1<Option<&'a mut T>>>::into_value_l1(t)
    }
}

impl<'a, T> IntoValueL1<'a, T> for Void where Void: IntoValueL2<'a, T> {
    #[inline] default fn into_value_l1(t: T) -> Result<Value<'a>, TError> {
        <Void as IntoValueL2<'a, T>>::into_value_l2(t)
    }
}

impl<'a, T> IntoValueL1<'a, Option<T>> for Void where Void: IntoValueL2<'a, T> {
    #[inline] fn into_value_l1(t: Option<T>) -> Result<Value<'a>, TError> {
        if let Some(t) = t {
            <Void as IntoValueL2<'a, T>>::into_value_l2(t)
        } else {
            Ok(Value::null_ptr())
        }
    }
}

impl<'a, T> IntoValueL2<'a, T> for Void where T: 'static {
    #[inline] default fn into_value_l2(t: T) -> Result<Value<'a>, TError> {
        <Void as IntoValueL3<'a, T>>::into_value_l3(t)
    }
}

impl<'a, T> IntoValueL2<'a, &'a T> for Void where T: 'static {
    #[inline] fn into_value_l2(t: &'a T) -> Result<Value<'a>, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::shared(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl<'a, T> IntoValueL2<'a, &'a mut T> for Void where T: 'static {
    #[inline] fn into_value_l2(t: &'a mut T) -> Result<Value<'a>, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::mut_shared(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl<'a, T> IntoValueL3<'a, T> for Void where T: 'static {
    #[inline] default fn into_value_l3(t: T) -> Result<Value<'a>, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::owned(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl<'a> IntoValueL3<'a, i64> for Void {
    #[inline] fn into_value_l3(t: i64) -> Result<Value<'a>, TError> {
        Ok(Value::from(t))
    }
}
