use std::mem::MaybeUninit;

use crate::data::{Value, GcInfo};
use crate::error::{TError, NullError, LifetimeError};
use crate::void::Void;
use crate::tyck::base::StaticBase;
use crate::tyck::FFIAction;

pub trait FromValue<'a, T> {
    fn lifetime_check(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value(value: Value<'a>) -> T;
}

pub trait FromValueL1<'a, T> {
    unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value_l1(value: Value<'a>) -> T;
}

pub trait FromValueL2<'a, T> {
    unsafe fn lifetime_check_l2(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value_l2(value: Value<'a>, out: &mut MaybeUninit<T>);
}

pub trait FromValueL3<'a, T> {
    unsafe fn lifetime_check_l3(value: &Value<'a>) -> Result<Option<GcInfo>, TError>;
    unsafe fn from_value_l3(value: Value<'a>, out: &mut MaybeUninit<T>);
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

const INTO_REF_LIFETIMES: [GcInfo; 2] = [GcInfo::Owned, GcInfo::SharedWithHost];
impl<'a, T> FromValueL1<'a, &'a T> for Void where Void: FromValueL2<'a, T> {
    unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        debug_assert!(!value.is_null());
        if value.is_ptr() {
            let actual = value.data.ptr.as_ref().unwrap().gc_info();
            if actual == GcInfo::Owned || actual == GcInfo::SharedWithHost {
                Ok(Some(actual))
            } else {
                Err(LifetimeError::new(&INTO_REF_LIFETIMES,
                                       FFIAction::Share,
                                       actual).into())
            }
        } else {
            Err(LifetimeError::new(&INTO_REF_LIFETIMES,
                                   FFIAction::Share,
                                   GcInfo::OnStack).into())
        }
    }

    unsafe fn from_value_l1(value: Value<'a>) -> &'a T {
        debug_assert!(!value.is_null());
        value.as_ref()
    }
}

const INTO_MUT_REF_LIFETIMES: [GcInfo; 1] = [GcInfo::Owned];
impl<'a, T> FromValueL1<'a, &'a mut T> for Void where Void: FromValueL2<'a, T> {
    unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        debug_assert!(!value.is_null());
        if value.is_ptr() {
            let actual = value.data.ptr.as_ref().unwrap().gc_info();
            if actual == GcInfo::Owned {
                Ok(Some(actual))
            } else {
                Err(LifetimeError::new(&INTO_MUT_REF_LIFETIMES,
                                       FFIAction::MutShare,
                                       GcInfo::OnStack).into())
            }
        } else {
            Err(LifetimeError::new(&INTO_MUT_REF_LIFETIMES,
                                   FFIAction::MutShare,
                                   GcInfo::OnStack).into())
        }
    }

    #[inline] unsafe fn from_value_l1(value: Value<'a>) -> &'a mut T {
        debug_assert!(!value.is_null());
        value.as_mut()
    }
}

impl<'a, T> FromValueL2<'a, T> for Void where Void: FromValueL3<'a, T> {
    #[inline] default unsafe fn lifetime_check_l2(_value: &Value<'a>)
        -> Result<Option<GcInfo>, TError>
    {
        unimplemented!()
    }

    #[inline] default unsafe fn from_value_l2(_value: Value<'a>, _out: &mut MaybeUninit<T>) {
        unimplemented!()
    }
}

impl<'a, T> FromValueL3<'a, T> for Void where Void: StaticBase<T> {
    #[inline] default unsafe fn lifetime_check_l3(_value: &Value<'a>)
        -> Result<Option<GcInfo>, TError>
    {
        unimplemented!()
    }

    #[inline] default unsafe fn from_value_l3(_value: Value<'a>, _out: &mut MaybeUninit<T>) {
        unimplemented!()
    }
}

impl<'a, T> FromValueL3<'a, T> for Void where Void: StaticBase<T>, T: Copy {
    unsafe fn lifetime_check_l3(_value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        unimplemented!()
    }

    unsafe fn from_value_l3(_value: Value<'a>, _out: &mut MaybeUninit<T>) {
        unimplemented!()
    }
}
