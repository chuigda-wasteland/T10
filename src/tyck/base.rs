use std::any::TypeId;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::void::Void;

pub trait StaticBase<T: 'static> {
    fn tyck_info() -> TypeCheckInfo;
    fn tyck(tyck_info: &TypeCheckInfo) -> bool;
    fn ffi_action() -> FFIAction;
}

trait StaticBaseImpl<T> {
    fn ffi_action_impl() -> FFIAction;
}

impl<T: 'static> StaticBase<T> for Void {
    default fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::SimpleType(TypeId::of::<T>())
    }

    default fn tyck(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::SimpleType(tid) = tyck_info {
            *tid == TypeId::of::<T>()
        } else {
            false
        }
    }

    default fn ffi_action() -> FFIAction {
        <Void as StaticBaseImpl<T>>::ffi_action_impl()
    }
}

impl<T: 'static> StaticBase<Vec<T>> for Void {
    fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::Container(
            TypeId::of::<Vec<Void>>(),
            vec![
                <Void as StaticBase<T>>::tyck_info()
            ]
        )
    }

    fn tyck(tyck_info: &TypeCheckInfo) -> bool {
        if let TypeCheckInfo::Container(container_tid, sub_infos) = tyck_info {
            *container_tid == TypeId::of::<Vec<Void>>()
            && sub_infos.len() == 1
            && <Void as StaticBase<T>>::tyck(
                unsafe { sub_infos.get_unchecked(0) }
            )
        } else {
            false
        }
    }

    fn ffi_action() -> FFIAction {
        FFIAction::Move
    }
}

impl<T> StaticBaseImpl<T> for Void {
    default fn ffi_action_impl() -> FFIAction {
        FFIAction::Move
    }
}

impl<T: Copy> StaticBaseImpl<T> for Void {
    fn ffi_action_impl() -> FFIAction {
        FFIAction::Copy
    }
}
