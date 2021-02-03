//! `data` 模块实现了 T10 中数据的实际存储形式。
//!
//! T10 内存模型的文档可以看[这里](https://github.com/Pr47/T10/issues/8#issuecomment-739257424)
//!
//! > TODO 需要更新文档部分

use std::any::{TypeId, type_name};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;

use crate::tyck::TypeCheckInfo;
use crate::tyck::base::StaticBase;
use crate::util::FatPointer;
use crate::void::Void;

/// 堆上对象的状态
///
/// T10 在与 Rust 交互时，可以将虚拟机中的对象移动、借用、可变借用给 Rust。因此相比其他的虚拟机，
/// T10 的堆对象拥有更多种状态
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GcInfo {
    /// T10 虚拟机拥有这个对象
    Owned = 0,
    /// 这个对象正在 Rust 和 T10 之间共享
    SharedWithHost = 1,
    /// 这个对象正在 Rust 和 T10 之间可变共享
    MutSharedWithHost = 2,
    /// 这个对象已经被移动到 Rust 一侧
    MovedToHost = 3,
    /// 这个对象已被回收
    Dropped = 4,
    /// 这是一个空对象
    Null = 5,
    /// 这是一个栈上对象
    OnStack = 6,
}

impl TryFrom<u8> for GcInfo {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => GcInfo::Owned,
            1 => GcInfo::SharedWithHost,
            2 => GcInfo::MutSharedWithHost,
            3 => GcInfo::MovedToHost,
            4 => GcInfo::Dropped,
            5 => GcInfo::Null,
            6 => GcInfo::OnStack,
            _ => return Err(())
        })
    }
}

union WrapperData<T> {
    value: ManuallyDrop<MaybeUninit<T>>,
    ptr: NonNull<T>
}

#[repr(align(8))]
pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    data: WrapperData<Ta>,
    gc_info: u8,
    _phantom: PhantomData<&'a Ts>
}

impl<'a, Ta: 'a, Ts: 'static> Wrapper<'a, Ta, Ts> {
    pub fn owned(data: Ta) -> Self {
        Self {
            data: WrapperData {
                value: ManuallyDrop::new(MaybeUninit::new(data))
            },
            gc_info: GcInfo::Owned as u8,
            _phantom: PhantomData::default()
        }
    }

    pub fn shared(data: &Ta) -> Self {
        Self {
            data: WrapperData {
                ptr: unsafe { NonNull::new_unchecked(data as *const Ta as *mut Ta) }
            },
            gc_info: GcInfo::SharedWithHost as u8,
            _phantom: PhantomData::default()
        }
    }

    pub fn mut_shared(data: &mut Ta) -> Self {
        Self {
            data: WrapperData {
                ptr: unsafe { NonNull::new_unchecked(data as *mut Ta) }
            },
            gc_info: GcInfo::MutSharedWithHost as u8,
            _phantom: PhantomData::default()
        }
    }

    #[inline] pub unsafe fn borrow_value(&self) -> NonNull<()> {
        NonNull::new_unchecked(self.data.value.as_ptr() as *const () as *mut ())
    }

    #[inline] pub unsafe fn borrow_ptr(&self) -> NonNull<()> {
        self.data.ptr.cast()
    }

    #[inline] pub unsafe fn take_value(&mut self) -> Ta {
        ManuallyDrop::take(&mut self.data.value).assume_init()
    }

    #[inline] fn gc_info_impl(&self) -> GcInfo {
        GcInfo::try_from(self.gc_info).unwrap()
    }

    #[inline] fn set_gc_info_impl(&mut self, gc_info: GcInfo) {
        self.gc_info = gc_info as u8
    }
}

impl<'a, Ta: 'a, Ts: 'static> Drop for Wrapper<'a, Ta, Ts> {
    fn drop(&mut self) {
        if false /* TODO use the real condition here */ {
            unsafe {
                ManuallyDrop::take(&mut self.data.value).assume_init_drop();
            }
        }
    }
}

pub type StaticWrapper<T> = Wrapper<'static, T, T>;

/// 负责在运行时从 `Wrapper` 中提取信息
///
/// 从方法签名上来看，`DynBase` 的所有方法都带有 `self` 参数，而 `StaticBased` 没有。
/// `StaticBase` 偏向于“编译”期的检查工作，而 `DynBase` 更偏向于运行时的动态分发。
pub trait DynBase {
    /// 获取类型 ID
    fn dyn_type_id(&self) -> std::any::TypeId;
    /// 获取类型名
    fn dyn_type_name(&self) -> &'static str;
    /// 运行时类型检测
    fn dyn_tyck(&self, tyck_info: &TypeCheckInfo) -> bool;
    /// 运行时获取类型检查信息
    fn dyn_tyck_info(&self) -> TypeCheckInfo;

    /// 获取 GC 信息
    fn gc_info(&self) -> GcInfo;
    /// 设置 GC 信息
    fn set_gc_info(&mut self, gc_info: GcInfo);

    /// 获取指向实际数据的指针
    unsafe fn get_ptr(&self) -> NonNull<()>;

    /// 将数据移动到 dest 中。dest 应为一个 `MaybeUninit`
    #[cfg(not(debug_assertions))]
    unsafe fn move_out(&mut self, dest: *mut ());

    /// 将数据移动到 dest中。dest 应为一个 `MaybeUninit`
    ///
    /// 这是带有运行时类型检查的版本，在 debug 模式下使用
    #[cfg(debug_assertions)]
    unsafe fn move_out_ck(&mut self, dest: *mut (), dest_ty: TypeId);
}

impl<'a, Ta: 'a, Ts: 'static> DynBase for Wrapper<'a, Ta, Ts> {
    fn dyn_type_id(&self) -> TypeId {
        TypeId::of::<Ts>()
    }

    fn dyn_type_name(&self) -> &'static str {
        type_name::<Ta>()
    }

    fn dyn_tyck(&self, tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<Ts>>::tyck(tyck_info)
    }

    fn dyn_tyck_info(&self) -> TypeCheckInfo {
        <Void as StaticBase<Ts>>::tyck_info()
    }

    fn gc_info(&self) -> GcInfo {
         self.gc_info_impl()
    }

    fn set_gc_info(&mut self, gc_info: GcInfo) {
        self.set_gc_info_impl(gc_info)
    }

    unsafe fn get_ptr(&self) -> NonNull<()> {
        match GcInfo::try_from(self.gc_info).unwrap() {
            GcInfo::Owned => self.borrow_value(),
            GcInfo::SharedWithHost | GcInfo::MutSharedWithHost => self.borrow_ptr(),
            GcInfo::MovedToHost => unreachable!("cannot use moved value"),
            GcInfo::Dropped => unreachable!("cannot use dropped value"),
            GcInfo::Null => unreachable!("null pointer should not occur at this layer"),
            GcInfo::OnStack => unreachable!("stack value should not occur at this layer")
        }
    }

    #[cfg(not(debug_assertions))]
    unsafe fn move_out(&mut self, dest: *mut ()) {
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.take_value());
    }

    #[cfg(debug_assertions)]
    unsafe fn move_out_ck(&mut self, dest: *mut (), dest_ty: TypeId) {
        debug_assert_eq!(self.gc_info(), GcInfo::Owned);
        debug_assert_eq!(dest_ty, TypeId::of::<MaybeUninit<Ts>>());
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.take_value());
    }
}

/// “值类型对象”的类型标记
#[derive(Copy, Clone)]
pub enum ValueType {
    Int     = 0b00000100,
    Float   = 0b00001000,
    Char    = 0b00001100,
    Byte    = 0b00010000,
    Bool    = 0b00010100,
    AnyType = 0b00011000
}

impl From<u8> for ValueType {
    fn from(src: u8) -> Self {
        match src {
            1 => ValueType::Int,
            2 => ValueType::Float,
            3 => ValueType::Char,
            4 => ValueType::Byte,
            5 => ValueType::Bool,
            6 => ValueType::AnyType,
            _ => unreachable!("invalid ValueType")
        }
    }
}

pub(crate) const VALUE_MASK      : u8 = 0b00000001;
pub(crate) const NULL_MASK       : u8 = 0b00000010;
pub(crate) const VALUE_TYPE_MASK : u8 = 0b01111100;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ValueTypedDataInner {
    /// 整数
    pub(crate) int: i64,
    /// 浮点数
    pub(crate) float: f64,
    /// 字符
    pub(crate) ch: char,
    /// 字节
    pub(crate) byte: u8,
    /// 布尔
    pub(crate) boolean: bool
}

#[derive(Copy, Clone)]
pub struct ValueTypedData {
    pub(crate) tag: usize,
    pub(crate) inner: ValueTypedDataInner
}

/// 一个通用的“值”
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    /// 堆对象指针
    pub(crate) ptr: *mut dyn DynBase,
    /// 堆对象指针的低层表示
    pub(crate) ptr_inner: FatPointer,
    /// 值类型数据
    pub(crate) value_typed_data: ValueTypedData
}

impl Value {
    /// 创建一个空指针
    #[inline] pub fn null_ptr() -> Self {
        Self {
            ptr_inner: FatPointer::null()
        }
    }

    /// 创建一个空值
    ///
    /// 由于 Pr47 采用值类型+引用类型的，空值和空指针并不是等同的概念。请见
    /// <https://github.com/Pr47/doc47/issues/9>
    ///
    /// > TODO 需要更新文档部分
    #[inline] pub fn null_value(value_type: ValueType) -> Self {
        Self {
            ptr_inner: FatPointer::from_parts(
                (value_type as u8 | VALUE_MASK | NULL_MASK) as usize,
                0
            )
        }
    }

    #[inline] pub fn is_null(&self) -> bool {
        unimplemented!()
    }

    #[inline] pub fn is_value(&self) -> bool {
        unimplemented!()
    }

    #[inline] pub fn is_ptr(&self) -> bool {
        unimplemented!()
    }

    #[inline] pub fn type_id(&self) -> TypeId {
        unimplemented!()
    }

    pub fn gc_info(&self) -> GcInfo {
        unimplemented!()
    }

    #[inline] pub unsafe fn set_gc_info(&self, _gc_info: GcInfo) {
        unimplemented!()
    }

    #[inline] pub unsafe fn as_ref<T>(&self) -> &T {
        unimplemented!()
    }

    #[inline] pub unsafe fn as_mut<T>(&self) -> &mut T {
        unimplemented!()
    }
}

impl<'a> From<*mut dyn DynBase> for Value {
    fn from(ptr: *mut dyn DynBase) -> Self {
        Self { ptr }
    }
}

impl<'a> From<i64> for Value {
    fn from(_int: i64) -> Self {
        unimplemented!()
    }
}

impl<'a> From<f64> for Value {
    fn from(_float: f64) -> Self {
        unimplemented!()
    }
}

impl<'a> From<char> for Value {
    fn from(_ch: char) -> Self {
        unimplemented!()
    }
}

impl<'a> From<bool> for Value {
    fn from(_boolean: bool) -> Self {
        unimplemented!()
    }
}

impl<'a> From<u8> for Value {
    fn from(_byte: u8) -> Self {
        unimplemented!()
    }
}
