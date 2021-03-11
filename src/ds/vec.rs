use std::any::TypeId;
use std::marker::PhantomData;
use crate::data::{CustomVTable, DynBase};
use crate::ds::ContainerElement;
use crate::tyck::TypeCheckInfo;

#[repr(C, align(8))]
pub struct VecVTable {
    pub base: CustomVTable,
    pub element: ContainerElement,
    pub nested: bool
}

pub fn vec_type_id(_vt: *const CustomVTable) -> TypeId {
    TypeId::of::<VMGenericVec>()
}

pub fn vec_type_name(vt: *const CustomVTable) -> String {
    let vec_vt = vt as *const VecVTable;
    unsafe {
        if (*vec_vt).nested {
            let elem_vt = (*vec_vt).element.element_vtable;
            format!("VMVec<{}>", ((*elem_vt).dyn_type_name)(elem_vt))
        } else {
            format!("VMVec<{}>", (*vec_vt).element.element_type_id.1.to_string())
        }
    }
}

pub fn vec_tyck(vt: *const CustomVTable, tyck_info: &TypeCheckInfo) -> bool {

}

#[repr(C)]
pub struct VMGenericVec {
    // 实现对 VMGenericVec 的操作需要先实现分配对象所需要的 VMContext
    // VMGenericVec 中存储的对象只能创建在 VMContext 上
    pub vec: Vec<*mut dyn DynBase>,
}

impl VMGenericVec {
    pub fn new() -> Self {
        Self {
            vec: Vec::new()
        }
    }
}

#[repr(transparent)]
pub struct VMVec<T> {
    // 实现对 VMVec 的操作需要先实现分配对象所需要的 VMContext
    // VMVec 中存储的对象只能创建在 VMContext 上
    #[allow(dead_code)]
    pub(crate) inner: VMGenericVec,
    _phantom: PhantomData<T>
}

impl<T> VMVec<T> {
    pub fn new() -> Self {
        Self {
            inner: VMGenericVec::new(),
            _phantom: PhantomData::default()
        }
    }
}
