use std::any::TypeId;
use std::marker::PhantomData;

pub enum Storage {
    VMOwned,
    SharedWithHost,
    MutSharedWithHost
}

pub enum RustArgStrategy {
    Move, Copy, Share, MutShare
}

pub struct Ptr<'a> {
    pub data: *mut (),
    pub type_info: TypeId,
    pub storage: Storage,
    _phantom: PhantomData<&'a ()>
}

impl<'a> Ptr<'a> {
    pub fn new(data: *mut (), type_info: TypeId, storage: Storage) -> Self {
        Self {
            data,
            type_info,
            storage,
            _phantom: PhantomData::default()
        }
    }
}

pub trait RustCallable<'a> {
    fn is_unsafe(&self) -> bool;

    fn param_specs(&self) -> &'static [(TypeId, RustArgStrategy)];

    unsafe fn call_prechecked(&self, args: &[Ptr<'a>]) -> Ptr<'a>;

    fn call(&self, args: &[Ptr<'a>]) -> Result<Ptr<'a>, &'static str> {
        let param_spec = self.param_specs();
        if param_spec.len() != args.len() {
            return Err("incorrect argument count")
        }

        for (arg, (param_type, param_strategy)) in args.iter().zip(param_spec.iter()) {
            if arg.type_info != *param_type {
                return Err("incorrect argument type")
            }

            let _: PhantomData<i32> = match (&arg.storage, param_strategy) {
                (Storage::VMOwned, _) => PhantomData::default(),
                (Storage::SharedWithHost, RustArgStrategy::Share) => PhantomData::default(),
                (Storage::MutSharedWithHost, RustArgStrategy::Share) => PhantomData::default(),
                (Storage::MutSharedWithHost, RustArgStrategy::MutShare) => PhantomData::default(),
                _ => return Err("other lifetime error")
            };
        }

        unsafe {
            Ok(self.call_prechecked(args))
        }
    }
}
