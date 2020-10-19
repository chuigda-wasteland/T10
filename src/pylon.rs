use std::sync::atomic::AtomicPtr;
use std::any::TypeId;
use std::marker::PhantomData;

pub struct Ptr {
    pub gc_info: AtomicPtr<u8>,
    pub data: *mut dyn DynBase
}

pub trait DynBase {
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
    fn maybe_type_name(&self) -> Option<&'static str>;
}

pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    pub inner: Ta,
    _phantom: PhantomData<(&'a (), Ts)>
}

pub type StaticWrapper<T> = Wrapper<'static, T, T>;

impl<'a, Ta: 'a, Ts: 'static> DynBase for Wrapper<'a, Ta, Ts> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Ts>()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Ts>()
    }

    fn maybe_type_name(&self) -> Option<&'static str> {
        None
    }
}
