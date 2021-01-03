use std::any::TypeId;
use std::marker::PhantomData;
use crate::data::DynBase;
use crate::tyck::base::StaticBase;
use crate::void::Void;
use crate::tyck::{TypeCheckInfo, FFIAction};

#[repr(transparent)]
pub struct VMGenericVec<'a> {
    // 实现对 VMGenericVec 的操作需要先实现分配对象所需要的 VMContext
    // VMGenericVec 中存储的对象只能创建在 VMContext 上
    #[allow(dead_code)]
    pub(crate) vec: Vec<*mut dyn DynBase>,
    _phantom: PhantomData<&'a ()>
}

impl<'a> VMGenericVec<'a> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            _phantom: PhantomData::default()
        }
    }
}

#[repr(transparent)]
pub struct VMVec<'a, T> {
    // 实现对 VMVec 的操作需要先实现分配对象所需要的 VMContext
    // VMVec 中存储的对象只能创建在 VMContext 上
    #[allow(dead_code)]
    pub(crate) inner: VMGenericVec<'a>,
    _phantom: PhantomData<T>
}

impl<'a, T> VMVec<'a, T> {
    pub fn new() -> Self {
        Self {
            inner: VMGenericVec::new(),
            _phantom: PhantomData::default()
        }
    }
}

impl<'a> StaticBase<VMGenericVec<'a>> for Void {
    fn base_type_id() -> TypeId {
        TypeId::of::<VMVec<Void>>()
    }

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
    fn base_type_id() -> TypeId {
        TypeId::of::<VMVec<Void>>()
    }

    fn tyck_info() -> TypeCheckInfo {
        TypeCheckInfo::Container(TypeId::of::<VMVec<Void>>(),
                                 vec![<Void as StaticBase<T>>::tyck_info()])
    }

    /*
    这里可能会比较费解：VMVec<T> 的 tyck 接受 VMVec<T> 和 VMGenericVec，而
    VMGenericVec 的 tyck 却仅接受 VMGenericVec

    调用 tyck 的一方永远是运行时才会生成的对象，提供 tyck_info 的一方是编译时可以确定的类型
    tyck 实际上是在询问 “能否将这个对象置入这个类型”。这样一来，VMVec<T> 可以被置入 VMGenericVec，
    反过来则不行
    */
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
