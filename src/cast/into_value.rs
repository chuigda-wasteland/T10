//! 从 Rust 对象到 `Value` 的转换

use std::error::Error;

use crate::data::{Value, StaticWrapper, DynBase};
use crate::error::TError;
use crate::void::Void;

/// 在这一层 specialization 中特殊处理 `Result<T, E>`
///
/// 对 `Result<T, E>` 的处理只在从 Rust 函数向 T10 运行时返回一个值的时候进行。
pub trait IntoValue<T> {
     fn into_value(t: T) -> Result<Value, TError>;
}

/// 在这一层的 specialization 中特殊处理 `&Option<T>` 和 `&mut Option<T>`
///
/// `&Option<T>` 或者 `&mut Option<T>` 会被进一步视为 `Option<&T>` 和 `Option<&mut T>`。
/// 这种特殊处理只会在从 Rust 函数向 T10 运行时返回一个值的时候进行。
pub trait IntoValueNoexcept<T> {
     fn into_value_noexcept(t: T) -> Result<Value, TError>;
}

/// 在这一层的 specialization 中特殊处理 `Option<T>`
pub trait IntoValueL1<T> {
     fn into_value_l1(t: T) -> Result<Value, TError>;
}

/// 在这一层的 specialization 中处理 `&T` 和 `&mut T`。
///
/// 从 Rust 环境向 T10 运行时传递的引用会被永久视为共享引用；禁止在 T10 通过 FFI 调用 Rust 函数时，
/// Rust 函数向 T10 返回任何引用。
pub trait IntoValueL2<T> {
     fn into_value_l2(t: T) -> Result<Value, TError>;
}

/// 在这一层的 specialization 中特殊处理 `i64` 之类的值类型
pub trait IntoValueL3<T> {
     fn into_value_l3(t: T) -> Result<Value, TError>;
}

impl<T> IntoValue<T> for Void where Void: IntoValueNoexcept<T> {
    default fn into_value(t: T) -> Result<Value, TError> {
        <Void as IntoValueNoexcept<T>>::into_value_noexcept(t)
    }
}

impl<T, E> IntoValue<Result<T, E>> for Void
    where Void: IntoValueNoexcept<T>,
          E: 'static + Error
{
    fn into_value(t: Result<T, E>) -> Result<Value, TError> {
        match t {
            Ok(data) => <Void as IntoValueNoexcept<T>>::into_value_noexcept(data),
            Err(e) => Err(TError::user_exception(e))
        }
    }
}

impl<T> IntoValueNoexcept<T> for Void where Void: IntoValueL1<T> {
    #[inline] default fn into_value_noexcept(t: T) -> Result<Value, TError> {
        <Void as IntoValueL1<T>>::into_value_l1(t)
    }
}

impl<T> IntoValueNoexcept<&'static Option<T>> for Void where T: 'static {
    #[inline] fn into_value_noexcept(t: &'static Option<T>) -> Result<Value, TError> {
        let t: Option<&'static T> = t.as_ref();
        <Void as IntoValueL1<Option<&'static T>>>::into_value_l1(t)
    }
}

impl<T> IntoValueNoexcept<&'static mut Option<T>> for Void where T: 'static {
    #[inline] fn into_value_noexcept(t: &'static mut Option<T>) -> Result<Value, TError> {
        let t: Option<&'static mut T> = t.as_mut();
        <Void as IntoValueL1<Option<&'static mut T>>>::into_value_l1(t)
    }
}

impl<T> IntoValueL1<T> for Void where Void: IntoValueL2<T> {
    #[inline] default fn into_value_l1(t: T) -> Result<Value, TError> {
        <Void as IntoValueL2<T>>::into_value_l2(t)
    }
}

impl<T> IntoValueL1<Option<T>> for Void where Void: IntoValueL2<T> {
    #[inline] fn into_value_l1(t: Option<T>) -> Result<Value, TError> {
        if let Some(t) = t {
            <Void as IntoValueL2<T>>::into_value_l2(t)
        } else {
            Ok(Value::null())
        }
    }
}

impl IntoValueL1<Value> for Void {
    #[inline] fn into_value_l1(t: Value) -> Result<Value, TError> {
        Ok(t)
    }
}

impl<T> IntoValueL2<T> for Void where T: 'static {
    #[inline] default fn into_value_l2(t: T) -> Result<Value, TError> {
        <Void as IntoValueL3<T>>::into_value_l3(t)
    }
}

impl<T> IntoValueL2<&'static T> for Void where T: 'static {
    #[inline] fn into_value_l2(t: &'static T) -> Result<Value, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::shared(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl<T> IntoValueL2<&'static mut T> for Void where T: 'static {
    #[inline] fn into_value_l2(t: &'static mut T) -> Result<Value, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::mut_shared(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl IntoValueL2<i64> for Void {
    #[inline] fn into_value_l2(t: i64) -> Result<Value, TError> {
        Ok(Value::from(t))
    }
}

impl<T> IntoValueL3<T> for Void where T: 'static {
    #[inline] default fn into_value_l3(t: T) -> Result<Value, TError> {
        let wrapper = Box::leak(Box::new(StaticWrapper::owned(t)));
        Ok(Value::from(wrapper as &mut dyn DynBase as *mut dyn DynBase))
    }
}

impl IntoValueL3<i64> for Void {
    #[inline] fn into_value_l3(t: i64) -> Result<Value, TError> {
        Ok(Value::from(t))
    }
}
