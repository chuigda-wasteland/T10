//! 本模块主要实现 Rust FFI 中 T10 `Value` 和 Rust 对象之间的相互转换
//!
//! 模块`into_value` 中存储的是将 Rust 对象存入 `Value` 的代码，Rust 对象存入 `Value` 可以发生在从
//! Rust 一侧调用 T10 函数时，或者 T10 调用的 Rust 函数返回时。
//!
//! 而模块 `from_value` 中存储的是将 `Value` 中的值取出，变成 Rust 对象的代码。`Value` 转换为 Rust
//! 对象可能发生在 从 T10 调用 Rust 函数时。

pub mod into_value;
pub mod from_value;
