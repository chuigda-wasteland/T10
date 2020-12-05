use std::any::TypeId;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::void::Void;

pub trait StaticBase<T> {
    fn base_type_id() -> TypeId;
    fn tyck_info() -> TypeCheckInfo;
    fn tyck(tyck_info: &TypeCheckInfo) -> bool;
    fn ffi_action() -> FFIAction;
}

trait StaticBaseImpl<T> {
    fn ffi_action_impl() -> FFIAction;
}

impl<T: 'static> StaticBase<T> for Void {
    #[inline] default fn base_type_id() -> TypeId {
        TypeId::of::<T>()
    }

    #[inline] default fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::SimpleType(TypeId::of::<T>())
    }

    #[inline] default fn tyck(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::SimpleType(tid) = tyck_info {
            *tid == TypeId::of::<T>()
        } else {
            false
        }
    }

    #[inline] default fn ffi_action() -> FFIAction {
        <Void as StaticBaseImpl<T>>::ffi_action_impl()
    }
}

impl<T> StaticBaseImpl<T> for Void {
    #[inline] default fn ffi_action_impl() -> FFIAction {
        FFIAction::Move
    }
}

impl<T: Copy> StaticBaseImpl<T> for Void {
    #[inline] fn ffi_action_impl() -> FFIAction {
        FFIAction::Copy
    }
}
