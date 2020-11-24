use std::error::Error;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::tyck::base::StaticBase;
use crate::void::Void;

pub trait FusionRV {
    fn tyck_info_rv() -> TypeCheckInfo;
    fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool;
    fn nullable_rv() -> bool;
    fn exception() -> bool;
    fn ffi_action_rv() -> FFIAction;
}

pub trait FusionRV2 {
    fn tyck_info_rv2() -> TypeCheckInfo;
    fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool;
    fn nullable_rv2() -> bool;
    fn ffi_action_rv2() -> FFIAction;
}

pub trait Fusion {
    fn fusion_tyck_info() -> TypeCheckInfo;
    fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool;
    fn nullable() -> bool;
    fn fusion_ffi_action() -> FFIAction;
}

pub trait Fusion2 {
    fn fusion_tyck_info2() -> TypeCheckInfo;
    fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool;
    fn fusion_ffi_action2() -> FFIAction;
}

impl<T: FusionRV2> FusionRV for T {
    default fn tyck_info_rv() -> TypeCheckInfo {
        <T as FusionRV2>::tyck_info_rv2()
    }

    default fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool {
        <T as FusionRV2>::tyck_rv2(tyck_info)
    }

    default fn nullable_rv() -> bool {
        <T as FusionRV2>::nullable_rv2()
    }

    default fn exception() -> bool {
        false
    }

    default fn ffi_action_rv() -> FFIAction {
        <T as FusionRV2>::ffi_action_rv2()
    }
}

impl<T: FusionRV2, E: 'static + Error> FusionRV for Result<T, E> {
    fn tyck_info_rv() -> TypeCheckInfo {
        <T as FusionRV2>::tyck_info_rv2()
    }

    fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool {
        <T as FusionRV2>::tyck_rv2(tyck_info)
    }

    fn nullable_rv() -> bool {
        <T as FusionRV2>::nullable_rv2()
    }

    fn exception() -> bool {
        true
    }

    fn ffi_action_rv() -> FFIAction {
        <T as FusionRV2>::ffi_action_rv2()
    }
}

impl<T: Fusion> FusionRV2 for T {
    default fn tyck_info_rv2() -> TypeCheckInfo {
        <T as Fusion>::fusion_tyck_info()
    }

    default fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <T as Fusion>::fusion_tyck(tyck_info)
    }

    default fn nullable_rv2() -> bool {
        <T as Fusion>::nullable()
    }

    default fn ffi_action_rv2() -> FFIAction {
        <T as Fusion>::fusion_ffi_action()
    }
}

impl<T: 'static> FusionRV2 for &Option<T> {
    fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    fn nullable_rv2() -> bool {
        true
    }

    fn ffi_action_rv2() -> FFIAction {
        FFIAction::Share
    }
}

impl<T: 'static> FusionRV2 for &mut Option<T> {
    fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    fn nullable_rv2() -> bool {
        true
    }

    fn ffi_action_rv2() -> FFIAction {
        FFIAction::MutShare
    }
}

impl<T: Fusion2> Fusion for T {
    default fn fusion_tyck_info() -> TypeCheckInfo {
        <T as Fusion2>::fusion_tyck_info2()
    }

    default fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <T as Fusion2>::fusion_tyck2(tyck_info)
    }

    default fn nullable() -> bool {
        false
    }

    default fn fusion_ffi_action() -> FFIAction {
        <T as Fusion2>::fusion_ffi_action2()
    }
}

impl<T: Fusion2> Fusion for Option<T> {
    fn fusion_tyck_info() -> TypeCheckInfo {
        <T as Fusion2>::fusion_tyck_info2()
    }

    fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <T as Fusion2>::fusion_tyck2(tyck_info)
    }

    fn nullable() -> bool {
        true
    }

    fn fusion_ffi_action() -> FFIAction {
        <T as Fusion2>::fusion_ffi_action2()
    }
}

impl<T: 'static> Fusion2 for T {
    default fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    default fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    default fn fusion_ffi_action2() -> FFIAction {
        <Void as StaticBase<T>>::ffi_action()
    }
}

impl<T: 'static> Fusion2 for &T {
    fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    fn fusion_ffi_action2() -> FFIAction {
        FFIAction::Share
    }
}

impl<T: 'static> Fusion2 for &mut T {
    fn fusion_tyck_info2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    fn fusion_tyck2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    fn fusion_ffi_action2() -> FFIAction {
        FFIAction::MutShare
    }
}

