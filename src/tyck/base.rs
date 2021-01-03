//! 本模块中实现了用于进行一系列“编译”期检查的 `StaticBase`

use std::any::TypeId;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::void::Void;

/// `StaticBase` 用于进行各类“编译”期检查，返回一些有用的信息
pub trait StaticBase<T> {
    /// 返回类型的“基础”类型 ID。对于一般类型而言，“基础”类型 ID 和 `std::any::TypeId::of`
    /// 取得的结果没有区别；对于**容器**类型而言，“基础”类型 ID 是包含 `crate::void::Void`
    /// 的容器类型的 `TypeId`。
    fn base_type_id() -> TypeId;

    /// 返回类型的完整类型检查信息
    fn tyck_info() -> TypeCheckInfo;

    /// 根据类型信息 `tyck_info`，判断 `T` 类型和 `tyck_info` 所指的类型是否兼容。
    fn tyck(tyck_info: &TypeCheckInfo) -> bool;

    /// 返回类型的 `FFIAction`
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
