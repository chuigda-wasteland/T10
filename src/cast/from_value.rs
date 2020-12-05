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
    #[inline] default fn from_value_l1(_value: Value<'a>) -> Result<T, String> {
        unimplemented!()
    }

    #[inline] default unsafe fn from_value_prechecked_l1(_value: Value<'a>) -> T {
        unimplemented!()
    }
}

impl<'a, T> FromValueL1<'a, &'a T> for Void where Void: FromValueL2<'a, T> {
    #[inline] fn from_value_l1(value: Value<'a>) -> Result<&'a T, String> {
        if value.is_value() {
            Err("cannot share a value".to_string())
        } else {
            unsafe {
                Ok(<Void as FromValueL1<'a, &'a T>>::from_value_prechecked_l1(value))
            }
        }
    }

    #[inline] unsafe fn from_value_prechecked_l1(value: Value<'a>) -> &'a T {
        // TODO runtime lifetime checking should still be performed
        // even with this "prechecked" line
        (value.data.ptr.as_ref().unwrap().as_ptr() as *mut T as *const T).as_ref().unwrap()
    }
}

impl<'a, T> FromValueL1<'a, &'a mut T> for Void where Void: FromValueL2<'a, T> {
    #[inline] fn from_value_l1(value: Value<'a>) -> Result<&'a mut T, String> {
        if value.is_value() {
            Err("cannot mutably share a value".to_string())
        } else {
            unsafe {
                Ok(<Void as FromValueL1<'a, &'a mut T>>::from_value_prechecked_l1(value))
            }
        }
    }

    #[inline] unsafe fn from_value_prechecked_l1(value: Value<'a>) -> &'a mut T {
        // TODO runtime lifetime checking should still be performed
        // even with this "prechecked" line
        (value.data.ptr.as_ref().unwrap().as_ptr() as *mut T).as_mut().unwrap()
    }
}

impl<'a, T> FromValueL2<'a, T> for Void where T: 'static {
    default fn from_value_l2(_value: Value<'a>) -> Result<T, String> {
        unimplemented!()
    }

    default unsafe fn from_value_prechecked_l2(_value: Value<'a>) -> T {
        unimplemented!()
    }
}
