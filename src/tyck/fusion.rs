//! `fusion` 模块用于实现 Rust FFI 时所需要的“编译期”类型检查

use std::any::TypeId;
use std::error::Error;

use crate::tyck::{TypeCheckInfo, FFIAction};
use crate::tyck::base::StaticBase;
use crate::void::Void;
use crate::data::Value;

pub type ExceptionSpec = Option<TypeId>;
pub type Nullable = bool;

/// ```compile_fail(E0477)
/// # use t10::void::Void;
/// # use t10::tyck::fusion::FusionRV;
/// fn test_compile_fail_rv<'a>(_x: &'a i64) {
///     let _ = <Void as FusionRV<&'a &'a i64>>::tyck_info_rv();
/// }
/// ```
pub trait FusionRV<T> {
    fn tyck_info_rv() -> TypeCheckInfo;
    fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool;
    fn tyck_type_rv<U>() -> bool;
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
    fn fusion_tyck_type<U>() -> bool;
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

    #[inline] default fn tyck_type_rv<U>() -> bool {
        unimplemented!()
    }

    #[inline] default fn ffi_action_rv() -> FFIAction {
        <Void as FusionRV2<T>>::ffi_action_rv2()
    }

    #[inline] default fn nullable_rv() -> bool {
        <Void as FusionRV2<T>>::nullable_rv2()
    }

    #[inline] default fn exception() -> ExceptionSpec {
        None
    }
}

impl<T, E> FusionRV<Result<T, E>> for Void where Void: FusionRV2<T>, E: 'static + Error {
    #[inline] fn tyck_info_rv() -> TypeCheckInfo {
        <Void as FusionRV2<T>>::tyck_info_rv2()
    }

    #[inline] fn tyck_rv(tyck_info: &TypeCheckInfo) -> bool {
        <Void as FusionRV2<T>>::tyck_rv2(tyck_info)
    }

    #[inline] fn tyck_type_rv<U>() -> bool {
        unimplemented!()
    }

    #[inline] fn ffi_action_rv() -> FFIAction {
        <Void as FusionRV2<T>>::ffi_action_rv2()
    }

    #[inline] fn nullable_rv() -> bool {
        <Void as FusionRV2<T>>::nullable_rv2()
    }

    #[inline] fn exception() -> ExceptionSpec {
        Some(TypeId::of::<E>())
    }
}

impl<T> FusionRV2<T> for Void where Void: Fusion<T> {
    #[inline] default fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as Fusion<T>>::fusion_tyck_info()
    }

    #[inline] default fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion<T>>::fusion_tyck(tyck_info)
    }

    #[inline] default fn ffi_action_rv2() -> FFIAction {
        <Void as Fusion<T>>::fusion_ffi_action()
    }

    #[inline] default fn nullable_rv2() -> bool {
        <Void as Fusion<T>>::nullable()
    }
}

impl<T: 'static> FusionRV2<&Option<T>> for Void where Void: StaticBase<T> {
    #[inline] fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn ffi_action_rv2() -> FFIAction {
        FFIAction::Share
    }

    #[inline] fn nullable_rv2() -> bool {
        true
    }
}

impl<T: 'static> FusionRV2<&mut Option<T>> for Void where Void: StaticBase<T> {
    #[inline] fn tyck_info_rv2() -> TypeCheckInfo {
        <Void as StaticBase<T>>::tyck_info()
    }

    #[inline] fn tyck_rv2(tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<T>>::tyck(tyck_info)
    }

    #[inline] fn ffi_action_rv2() -> FFIAction {
        FFIAction::MutShare
    }

    #[inline] fn nullable_rv2() -> bool {
        true
    }
}

impl<T> Fusion<T> for Void where Void: Fusion2<T> {
    #[inline] default fn fusion_tyck_info() -> TypeCheckInfo {
        <Void as Fusion2<T>>::fusion_tyck_info2()
    }

    #[inline] default fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion2<T>>::fusion_tyck2(tyck_info)
    }

    #[inline] default fn fusion_tyck_type<U>() -> bool {
        unimplemented!()
    }

    #[inline] default fn fusion_ffi_action() -> FFIAction {
        <Void as Fusion2<T>>::fusion_ffi_action2()
    }

    #[inline] default fn nullable() -> bool {
        false
    }
}

impl<T> Fusion<Option<T>> for Void where Void: Fusion2<T> {
    #[inline] fn fusion_tyck_info() -> TypeCheckInfo {
        <Void as Fusion2<T>>::fusion_tyck_info2()
    }

    #[inline] fn fusion_tyck(tyck_info: &TypeCheckInfo) -> bool {
        <Void as Fusion2<T>>::fusion_tyck2(tyck_info)
    }

    #[inline] fn fusion_tyck_type<U>() -> bool {
        unimplemented!()
    }

    #[inline] fn fusion_ffi_action() -> FFIAction {
        <Void as Fusion2<T>>::fusion_ffi_action2()
    }

    #[inline] fn nullable() -> bool {
        true
    }
}

impl Fusion<Value> for Void {
    #[inline] fn fusion_tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::Bypass
    }

    #[inline] fn fusion_tyck(_tyck_info: &TypeCheckInfo) -> bool {
        true
    }

    #[inline] fn fusion_tyck_type<U>() -> bool {
        true
    }

    #[inline] fn fusion_ffi_action() -> FFIAction {
        FFIAction::Bypass
    }

    #[inline] fn nullable() -> bool {
        true
    }
}

impl<T> Fusion2<T> for Void where Void: StaticBase<T> {
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

impl<'a, T> Fusion2<&'a T> for Void where Void: StaticBase<T> {
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

impl<'a, T> Fusion2<&'a mut T> for Void where Void: StaticBase<T> {
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

#[cfg(test)]
mod test {
    use std::any::TypeId;
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    use crate::tyck::FFIAction;
    use crate::tyck::TypeCheckInfo;
    use crate::tyck::fusion::{ExceptionSpec, FusionRV};
    use crate::void::Void;

    fn type_check_info_assert(tyck_info: &TypeCheckInfo, type_ids: &[TypeId]) {
        debug_assert_ne!(type_ids.len(), 0);
        
        match tyck_info {
            &TypeCheckInfo::Bypass => {
                assert_eq!(type_ids.len(), 0);
            },
            &TypeCheckInfo::SimpleType(type_id) => {
                assert_eq!(type_ids.len(), 1);
                assert_eq!(type_ids[0], type_id);
            },
            TypeCheckInfo::Container(container_id, elements) => {
                assert_ne!(type_ids.len(), 1);
                assert_eq!(type_ids[0], *container_id);
                // TODO this is temporary, and will be removed after we add binary containers
                assert_eq!(elements.len(), 1);

                type_check_info_assert(&elements[0], &type_ids[1..]);
            }
        }
    }

    fn test_type_infos_rv<T>(
        type_id_sequence: &[TypeId],
        expected_ffi_action: FFIAction,
        expected_nullable: bool,
        expected_exception: ExceptionSpec
    ) where Void: FusionRV<T> {
        let tyck_info = <Void as FusionRV<T>>::tyck_info_rv();
        let ffi_action = <Void as FusionRV<T>>::ffi_action_rv();
        let nullable = <Void as FusionRV<T>>::nullable_rv();
        let exception = <Void as FusionRV<T>>::exception();

        type_check_info_assert(&tyck_info, type_id_sequence);
        assert_eq!(ffi_action, expected_ffi_action);
        assert_eq!(nullable, expected_nullable);
        assert_eq!(exception, expected_exception);
    }

    #[test]
    fn test_fusion_rv_simple() {
        test_type_infos_rv::<i64>(
            &[TypeId::of::<i64>()], FFIAction::Copy, false, None as ExceptionSpec
        );
    }

    #[test]
    fn test_fusion_rv_move() {
        test_type_infos_rv::<String>(
            &[TypeId::of::<String>()], FFIAction::Move, false, None as ExceptionSpec
        );
    }

    #[test]
    fn test_fusion_rv_ref() {
        test_type_infos_rv::<&i64>(
            &[TypeId::of::<i64>()], FFIAction::Share, false, None as ExceptionSpec
        );
        test_type_infos_rv::<&mut i64>(
            &[TypeId::of::<i64>()], FFIAction::MutShare, false, None as ExceptionSpec
        );
        test_type_infos_rv::<&String>(
            &[TypeId::of::<String>()], FFIAction::Share, false, None as ExceptionSpec
        );
        test_type_infos_rv::<&mut String>(
            &[TypeId::of::<String>()], FFIAction::MutShare, false, None as ExceptionSpec
        );
    }

    #[test]
    fn test_fusion_rv_nullable() {
        type TestedType1 = Option<i64>;
        type TestedType2 = Option<String>;
        type TestedType3<'a> = Option<&'a i64>;
        type TestedType4<'a> = Option<&'a mut i64>;
        type TestedType5<'a> = Option<&'a String>;
        type TestedType6<'a> = Option<&'a mut String>;

        test_type_infos_rv::<TestedType1>(
            &[TypeId::of::<i64>()], FFIAction::Copy, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType2>(
            &[TypeId::of::<String>()], FFIAction::Move, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType3>(
            &[TypeId::of::<i64>()], FFIAction::Share, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType4>(
            &[TypeId::of::<i64>()], FFIAction::MutShare, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType5>(
            &[TypeId::of::<String>()], FFIAction::Share, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType6>(
            &[TypeId::of::<String>()], FFIAction::MutShare, true, None as ExceptionSpec
        );
    }

    #[test]
    fn test_fusion_rv_nullable2() {
        type TestedType1<'a> = &'a Option<i64>;
        type TestedType2<'a> = &'a mut Option<i64>;
        type TestedType3<'a> = &'a Option<String>;
        type TestedType4<'a> = &'a mut Option<String>;

        test_type_infos_rv::<TestedType1>(
            &[TypeId::of::<i64>()], FFIAction::Share, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType2>(
            &[TypeId::of::<i64>()], FFIAction::MutShare, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType3>(
            &[TypeId::of::<String>()], FFIAction::Share, true, None as ExceptionSpec
        );
        test_type_infos_rv::<TestedType4>(
            &[TypeId::of::<String>()], FFIAction::MutShare, true, None as ExceptionSpec
        );
    }

    #[derive(Debug)] struct TestError1();
    #[derive(Debug)] struct TestError2();

    impl Display for TestError1 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "TestError1") }
    }

    impl Display for TestError2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "TestError2") }
    }

    impl Error for TestError1 {}
    impl Error for TestError2 {}

    #[test]
    fn test_fusion_rv_result() {
        type TestedType1 = Result<i64, TestError1>;
        type TestedType2 = Result<i64, TestError2>;

        test_type_infos_rv::<TestedType1>(
            &[TypeId::of::<i64>()], FFIAction::Copy, false, Some(TypeId::of::<TestError1>())
        );
        test_type_infos_rv::<TestedType2>(
            &[TypeId::of::<i64>()], FFIAction::Copy, false, Some(TypeId::of::<TestError2>())
        );
    }
}
