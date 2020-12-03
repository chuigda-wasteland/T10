use crate::data::Value;
use crate::void::Void;

pub trait FromValue<'a, T> {
    fn from_value(value: Value<'a>) -> Result<T, String>;
    unsafe fn from_value_prechecked(value: Value<'a>) -> T;
}

pub trait FromValueL1<'a, T> {
    fn from_value_l1(value: Value<'a>) -> Result<T, String>;
    unsafe fn from_value_prechecked_l1(value: Value<'a>) -> T;
}

pub trait FromValueL2<'a, T> {
    fn from_value_l2(value: Value<'a>) -> Result<T, String>;
    unsafe fn from_value_prechecked_l2(value: Value<'a>) -> T;
}

impl<'a, T> FromValue<'a, T> for Void where Void: FromValueL1<'a, T> {
    #[inline] default fn from_value(value: Value<'a>) -> Result<T, String> {
        if value.is_null() {
            Err("NullPointerException".to_string())
        } else {
            <Void as FromValueL1<'a, T>>::from_value_l1(value)
        }
    }

    #[inline] default unsafe fn from_value_prechecked(value: Value<'a>) -> T {
        debug_assert!(!value.is_null());
        <Void as FromValueL1<'a, T>>::from_value_prechecked_l1(value)
    }
}

impl<'a, T> FromValue<'a, Option<T>> for Void where Void: FromValueL1<'a, T> {
    #[inline] fn from_value(value: Value<'a>) -> Result<Option<T>, String> {
        if value.is_null() {
            Ok(None)
        } else {
            todo!()
        }
    }

    #[inline] unsafe fn from_value_prechecked(value: Value<'a>) -> Option<T> {
        if value.is_null() {
            None
        } else {
            todo!()
        }
    }
}

impl<'a, T> FromValueL1<'a, T> for Void where Void: FromValueL2<'a, T> {
    fn from_value_l1(_value: Value<'a>) -> Result<T, String> {
        unimplemented!()
    }

    unsafe fn from_value_prechecked_l1(_value: Value<'a>) -> T {
        unimplemented!()
    }
}
