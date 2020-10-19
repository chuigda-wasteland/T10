use std::sync::atomic::AtomicPtr;
use std::any::{TypeId, Any};
use std::marker::PhantomData;

pub enum GcInfo {
    OnVMStack         = 0,
    OnVMHeap          = 1,
    SharedWithHost    = 2,
    MutSharedWithHost = 3
}

pub struct Ptr<'a> {
    pub gc_info: AtomicPtr<GcInfo>,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>
}

impl<'a> Ptr<'a> {
    pub(crate) fn static_type_id(&self) -> TypeId {
        unsafe {
            self.data.as_ref().map_or_else(|| {
                PhantomNullPtr {}.static_type_id()
            }, |dyn_base| {
                dyn_base.type_id()
            })
        }
    }
}

pub trait DynBase {
    fn static_type_id(&self) -> TypeId;
    fn static_type_name(&self) -> &'static str;
    fn maybe_type_name(&self) -> Option<&'static str>;
}

pub struct PhantomNullPtr {}

impl DynBase for PhantomNullPtr {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<dyn DynBase>()
    }

    fn static_type_name(&self) -> &'static str {
        "any"
    }

    fn maybe_type_name(&self) -> Option<&'static str> {
        None
    }
}

pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    pub inner: Ta,
    _phantom: PhantomData<(&'a (), Ts)>
}

pub type StaticWrapper<T> = Wrapper<'static, T, T>;

impl<'a, Ta: 'a, Ts: 'static> DynBase for Wrapper<'a, Ta, Ts> {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<Ts>()
    }

    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Ts>()
    }

    fn maybe_type_name(&self) -> Option<&'static str> {
        None
    }
}
