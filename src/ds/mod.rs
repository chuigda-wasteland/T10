use std::marker::PhantomData;
use crate::data::DynBase;

#[repr(transparent)]
pub struct VMGenericVec {
    inner: *mut dyn DynBase
}

#[repr(transparent)]
pub struct VMVec<T> {
    inner: *mut dyn DynBase,
    _phantom: PhantomData<T>
}
