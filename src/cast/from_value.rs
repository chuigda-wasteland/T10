use std::mem::MaybeUninit;

use crate::data::{Value, GcInfo};
use crate::error::{TError, NullError, LifetimeError};
use crate::void::Void;
use crate::tyck::base::StaticBase;
use crate::tyck::FFIAction;
use std::any::TypeId;

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
    unsafe fn from_value_l2(value: Value<'a>) -> T;
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
        <Void as FromValueL2<'a, T>>::lifetime_check_l2(value)
    }

    #[inline] default unsafe fn from_value_l1(value: Value<'a>) -> T {
        debug_assert!(!value.is_null());
        <Void as FromValueL2<'a, T>>::from_value_l2(value)
    }
}

const INTO_REF_LIFETIMES: [GcInfo; 2] = [GcInfo::Owned, GcInfo::SharedWithHost];
impl<'a, T> FromValueL1<'a, &'a T> for Void where Void: FromValueL2<'a, T> {
    unsafe fn lifetime_check_l1(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        debug_assert!(!value.is_null());
        let actual = value.gc_info();
        if actual == GcInfo::Owned || actual == GcInfo::SharedWithHost {
            Ok(Some(actual))
        } else {
            Err(LifetimeError::new(&INTO_REF_LIFETIMES, FFIAction::Share, actual).into())
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
        let actual = value.gc_info();
        if actual == GcInfo::Owned {
            Ok(Some(actual))
        } else {
            Err(LifetimeError::new(&INTO_MUT_REF_LIFETIMES, FFIAction::MutShare, actual).into())
        }
    }

    #[inline] unsafe fn from_value_l1(value: Value<'a>) -> &'a mut T {
        debug_assert!(!value.is_null());
        value.as_mut()
    }
}

impl<'a, T> FromValueL2<'a, T> for Void where Void: FromValueL3<'a, T> {
    #[inline] default unsafe fn lifetime_check_l2(value: &Value<'a>)
        -> Result<Option<GcInfo>, TError>
    {
        <Void as FromValueL3<'a, T>>::lifetime_check_l3(value)
    }

    #[inline] default unsafe fn from_value_l2(value: Value<'a>) -> T {
        let mut ret: MaybeUninit<T> = MaybeUninit::uninit();
        <Void as FromValueL3<'a, T>>::from_value_l3(value, &mut ret);
        ret.assume_init()
    }
}

const VALUE_TYPE_LIFETIMES: [GcInfo; 4] = [
    GcInfo::Owned,
    GcInfo::SharedWithHost,
    GcInfo::MutSharedWithHost,
    GcInfo::OnStack
];
impl<'a> FromValueL2<'a, i64> for Void {
    #[inline] unsafe fn lifetime_check_l2(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        let actual = value.gc_info();
        if VALUE_TYPE_LIFETIMES.contains(&actual) {
            Ok(None)
        } else {
            Err(LifetimeError::new(&VALUE_TYPE_LIFETIMES, FFIAction::Copy, actual).into())
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline] unsafe fn from_value_l2(value: Value<'a>) -> i64 {
        if value.is_value() {
            value.data.int
        } else {
            let mut ret: MaybeUninit<i64> = MaybeUninit::uninit();
            value.data.ptr.as_mut().unwrap().move_out(
                &mut ret as *mut MaybeUninit<_> as *mut ()
            );
            ret.assume_init()
        }
    }

    #[cfg(debug_assertions)]
    #[inline] unsafe fn from_value_l2(value: Value<'a>) -> i64 {
        if value.is_value() {
            value.data.int
        } else {
            let mut ret: MaybeUninit<i64> = MaybeUninit::uninit();
            value.data.ptr.as_mut().unwrap().move_out_ck(
                &mut ret as *mut MaybeUninit<_> as *mut (),
                TypeId::of::<MaybeUninit<i64>>()
            );
            ret.assume_init()
        }
    }
}

const MOVE_TYPE_LIFETIMES: [GcInfo; 1] = [ GcInfo::Owned ];
impl<'a, T> FromValueL3<'a, T> for Void where Void: StaticBase<T> {
    #[inline] default unsafe fn lifetime_check_l3(value: &Value<'a>)
        -> Result<Option<GcInfo>, TError>
    {
        let actual = value.gc_info();
        if actual == GcInfo::Owned {
            Ok(None)
        } else {
            Err(LifetimeError::new(&MOVE_TYPE_LIFETIMES, FFIAction::Move, actual).into())
        }
    }

    #[inline] default unsafe fn from_value_l3(value: Value<'a>, out: &mut MaybeUninit<T>) {
        value.data.ptr.as_mut().unwrap().move_out_ck(
            out as *mut MaybeUninit<_> as *mut (),
            <Void as StaticBase<T>>::base_type_id()
        );
    }
}

impl<'a, T> FromValueL3<'a, T> for Void where Void: StaticBase<T>, T: Copy {
    unsafe fn lifetime_check_l3(value: &Value<'a>) -> Result<Option<GcInfo>, TError> {
        let actual = value.gc_info();
        if VALUE_TYPE_LIFETIMES.contains(&actual) {
            Ok(None)
        } else {
            Err(LifetimeError::new(&VALUE_TYPE_LIFETIMES, FFIAction::Copy, actual).into())
        }
    }

    unsafe fn from_value_l3(value: Value<'a>, out: &mut MaybeUninit<T>) {
        out.write(value.as_ref::<T>().clone());
    }
}
