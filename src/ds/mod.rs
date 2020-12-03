use std::any::TypeId;
use std::marker::PhantomData;
use crate::data::DynBase;
use crate::tyck::base::StaticBase;
use crate::void::Void;
use crate::tyck::{TypeCheckInfo, FFIAction};

#[repr(transparent)]
pub struct VMGenericVec<'a> {
    inner: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>
}

#[repr(transparent)]
pub struct VMVec<'a, T> {
    inner: VMGenericVec<'a>,
    _phantom: PhantomData<T>
}

impl<'a> StaticBase<VMGenericVec<'a>> for Void {
    fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::Container(TypeId::of::<VMVec<Void>>(), vec![])
    }

    fn tyck(tyck_info: &TypeCheckInfo) -> bool {
        match tyck_info {
            TypeCheckInfo::Container(container_type, type_params) => {
                *container_type == TypeId::of::<VMVec<Void>>()
                && type_params.len() == 0
            },
            _ => false
        }
    }

    #[inline] fn ffi_action() -> FFIAction {
        FFIAction::Move
    }
}

impl<'a, T> StaticBase<VMVec<'a, T>> for Void
    where Void: StaticBase<T>
{
    fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::Container(TypeId::of::<VMVec<Void>>(),
                                 vec![<Void as StaticBase<T>>::tyck_info()])
    }

    fn tyck(tyck_info: &TypeCheckInfo) -> bool {
        match tyck_info {
            TypeCheckInfo::Container(container_type, type_params) => {
                *container_type == TypeId::of::<VMVec<Void>>()
                && (type_params.len() == 1
                    && <Void as StaticBase<T>>::tyck(&type_params[0]))
                   || type_params.len() == 0
            },
            _ => false
        }
    }

    #[inline] fn ffi_action() -> FFIAction {
        FFIAction::Move
    }
}
