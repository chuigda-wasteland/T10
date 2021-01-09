//! `fusion` 模块用于实现 Rust FFI 时所需要的“编译期”类型检查

use std::any::TypeId;
use std::error::Error;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::tyck::base::StaticBase;
use crate::void::Void;

pub type ExceptionSpec = Option<TypeId>;
pub type Nullable = bool;

pub trait FusionRV<T> {
    fn tyck_info_rv() -> TypeCheckInfo;
    fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool;
    fn ffi_action_rv() -> FFIAction;

    fn nullable_rv() -> Nullable;
    fn exception() -> ExceptionSpec;
}

pub trait FusionRV2<T> {
    fn tyck_info_rv2() -> TypeCheckInfo;
    fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool;
    fn ffi_action_rv2() -> FFIAction;

    fn nullable_rv2() -> Nullable;
}

pub trait Fusion<T> {
    fn fusion_tyck_info() -> TypeCheckInfo;
    fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool;
    fn fusion_ffi_action() -> FFIAction;

    fn nullable() -> Nullable;
}

pub trait Fusion2<T> {
    fn fusion_tyck_info2() -> TypeCheckInfo;
    fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool;
    fn fusion_ffi_action2() -> FFIAction;
}

impl<T> FusionRV<T> for Void where Void: FusionRV2<T> {
    #[inline] default fn tyck_info_rv() -> TypeCheckInfo {
        <Void as FusionRV2<T>>::tyck_info_rv2()
    }

    #[inline] default fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool {
        <Void as FusionRV2<T>>::tyck_rv2(tyck_info)
    }

    #[inline] default fn nullable_rv() -> bool {
        <Void as FusionRV2<T>>::nullable_rv2()
    }

    #[inline] default fn exception() -> ExceptionSpec {
        None
    }

    #[inline] default fn ffi_action_rv() -> FFIAction {
        <Void as FusionRV2<T>>::ffi_action_rv2()
    }
}

impl<T, E> FusionRV<Result<T, E>> for Void where Void: FusionRV2<T>, E: 'static + Error {
    #[inline] fn tyck_info_rv() -> TypeCheckInfo {
        <Void as FusionRV2<T>>::tyck_info_rv2()
    }

    #[inline] fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool {
        <Void as FusionRV2<T>>::tyck_rv2(tyck_info)
    }

    #[inline] fn nullable_rv() -> bool {
        <Void as FusionRV2<T>>::nullable_rv2()
    }

    #[inline] fn exception() -> ExceptionSpec {
        Some(TypeId::of::<E>())
    }

    #[inline] fn ffi_action_rv() -> FFIAction {
        <Void as FusionRV2<T>>::ffi_action_rv2()
    }
}

impl<T> FusionRV2<T> for Void where Void: Fusion<T> {
    #[inline] default fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as Fusion<T>>::fusion_tyck_info()
    }

    #[inline] default fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion<T>>::fusion_tyck(tyck_info)
    }

    #[inline] default fn nullable_rv2() -> bool {
        <Void as Fusion<T>>::nullable()
    }

    #[inline] default fn ffi_action_rv2() -> FFIAction {
        <Void as Fusion<T>>::fusion_ffi_action()
    }
}

impl<T: 'static> FusionRV2<&Option<T>> for Void where Void: StaticBase<T> {
    #[inline] fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn nullable_rv2() -> bool {
        true
    }

    #[inline] fn ffi_action_rv2() -> FFIAction {
        FFIAction::Share
    }
}

impl<T: 'static> FusionRV2<&mut Option<T>> for Void where Void: StaticBase<T> {
    #[inline] fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn nullable_rv2() -> bool {
        true
    }

    #[inline] fn ffi_action_rv2() -> FFIAction {
        FFIAction::MutShare
    }
}

impl<T> Fusion<T> for Void where Void: Fusion2<T> {
    #[inline] default fn fusion_tyck_info() -> TypeCheckInfo {
        <Void as Fusion2<T>>::fusion_tyck_info2()
    }

    #[inline] default fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion2<T>>::fusion_tyck2(tyck_info)
    }

    #[inline] default fn nullable() -> bool {
        false
    }

    #[inline] default fn fusion_ffi_action() -> FFIAction {
        <Void as Fusion2<T>>::fusion_ffi_action2()
    }
}

impl<T> Fusion<Option<T>> for Void where Void: Fusion2<T> {
    #[inline] fn fusion_tyck_info() -> TypeCheckInfo {
        <Void as Fusion2<T>>::fusion_tyck_info2()
    }

    #[inline] fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion2<T>>::fusion_tyck2(tyck_info)
    }

    #[inline] fn nullable() -> bool {
        true
    }

    #[inline] fn fusion_ffi_action() -> FFIAction {
        <Void as Fusion2<T>>::fusion_ffi_action2()
    }
}

impl<T: 'static> Fusion2<T> for Void where Void: StaticBase<T> {
    #[inline] default fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] default fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] default fn fusion_ffi_action2() -> FFIAction {
        <Void as StaticBase<T>>::ffi_action()
    }
}

impl<'a, T: 'static> Fusion2<&'a T> for Void where Void: StaticBase<T> {
    #[inline] fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn fusion_ffi_action2() -> FFIAction {
        FFIAction::Share
    }
}

impl<'a, T: 'static> Fusion2<&'a mut T> for Void where Void: StaticBase<T> {
    #[inline] fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn fusion_ffi_action2() -> FFIAction {
        FFIAction::MutShare
    }
}
