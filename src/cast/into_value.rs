use std::error::Error;

use crate::data::{Value, StaticWrapper, DynBase};
use crate::error::TError;
use crate::void::Void;

pub trait IntoValue<'a, T> {
     fn into_value(t: T) -> Result<Value<'a>, TError>;
}

pub trait IntoValueNoexcept<'a, T> {
     fn into_value_noexcept(t: T) -> Result<Value<'a>, TError>;
}

pub trait IntoValueL1<'a, T> {
     fn into_value_l1(t: T) -> Result<Value<'a>, TError>;
}

pub trait IntoValueL2<'a, T> {
     fn into_value_l2(t: T) -> Result<Value<'a>, TError>;
}

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
    fn into_value_l3(t: i64) -> Result<Value<'a>, TError> {
        Ok(Value::from(t))
    }
}
