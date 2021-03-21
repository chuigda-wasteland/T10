//! 此模块用来存储供 T10 使用的泛型容器，细节调整尚未完成

pub mod object;
pub mod value_object;
pub mod vec;
pub mod value_vec;

use std::any::TypeId;
use crate::data::CustomVTable;

#[repr(C)]
pub union ContainerElement {
    element_type_id: (TypeId, &'static str),
    element_vtable: *const CustomVTable
}
