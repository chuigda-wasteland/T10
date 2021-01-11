//! 本模块主要实现与 Rust FFI 相关的类型检查和“生存期检查”

pub mod base;
pub mod fusion;

/// 类型检查信息
///
/// 类型检查信息在“编译”时生成，运行时用来确定两个类型之间的兼容性。目前的实现采用朴素的分配方式，
/// 之后可能改用一个 arena 来管理所有的 `TypeCheckInfo`。
#[derive(Debug)]
pub enum TypeCheckInfo {
    /// 简单类型
    SimpleType(std::any::TypeId),
    /// 容器类型
    Container(std::any::TypeId, Vec<TypeCheckInfo>),
}

/// 生存期检查信息
///
/// 生存期检查信息在“编译”时生成，运行时用来确定对象如何在 Rust 和 `T10` 之间传递
#[derive(Debug, Eq, PartialEq)]
pub enum FFIAction {
    /// 移动
    Move,
    /// 拷贝
    Copy,
    /// 共享
    Share,
    /// 可变共享
    MutShare
}
