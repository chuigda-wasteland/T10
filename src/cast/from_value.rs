use crate::data::{Value, GcInfo};
use crate::void::Void;
use crate::error::{TError, NullError};
use crate::tyck::base::StaticBase;

pub trait FromValue<'a, T> {
    fn lifetime_check(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value(value: Value<'a>) -> T;
}

pub trait FromValueL1<'a, T> {
    unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value_l1(value: Value<'a>) -> T;
}

pub trait FromValueL2<'a, T> {
    // TODO what do we need in FromValueL2
}

impl<'a, T> FromValue<'a, T> for Void where Void: FromValueL1<'a, T> {
    #[inline] default fn lifetime_check(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        if value.is_null() {
            Err(NullError().into())
        } else {
            unsafe { <Void as FromValueL1<'a, T>>::lifetime_check_l1(value) }
        }
    }

    #[inline] default unsafe fn from_value(value: Value<'a>) -> T {
        <Void as FromValueL1<'a, T>>::from_value_l1(value)
    }
}

impl<'a, T> FromValue<'a, Option<T>> for Void where Void: FromValueL1<'a, T> {
    #[inline] fn lifetime_check(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        if value.is_null() {
            Ok(None)
        } else {
            unsafe { <Void as FromValueL1<'a, T>>::lifetime_check_l1(value) }
        }
    }

    #[inline] unsafe fn from_value(value: Value<'a>) -> Option<T> {
        Some(<Void as FromValueL1<'a, T>>::from_value_l1(value))
    }
}

impl<'a, T> FromValueL1<'a, T> for Void where Void: FromValueL2<'a, T> {
    #[inline] default unsafe fn lifetime_check_l1(value: &Value<'a>)
        -> Result<Option<GcInfo>, TError>
    {
        debug_assert!(!value.is_null());
        unimplemented!()
    }

    #[inline] default unsafe fn from_value_l1(value: Value<'a>) -> T {
        debug_assert!(!value.is_null());
        unimplemented!()
    }
}

impl<'a, T> FromValueL1<'a, &'a T> for Void where Void: FromValueL2<'a, T> {
    #[inline] unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        debug_assert!(!value.is_null());
        unimplemented!()
    }

    #[inline] unsafe fn from_value_l1(value: Value<'a>) -> &'a T {
        debug_assert!(!value.is_null());
        unimplemented!()
    }
}

impl<'a, T> FromValueL1<'a, &'a mut T> for Void where Void: FromValueL2<'a, T> {
    #[inline] unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        debug_assert!(!value.is_null());
        unimplemented!()
    }

    #[inline] unsafe fn from_value_l1(value: Value<'a>) -> &'a mut T {
        debug_assert!(!value.is_null());
        unimplemented!()
    }
}

impl<'a, T> FromValueL2<'a, T> for Void where Void: StaticBase<T> {}
