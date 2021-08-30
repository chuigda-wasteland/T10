//! `data` 模块实现了 T10 中数据的实际存储形式。
//!
//! T10 内存模型的文档可以看[这里](https://github.com/Pr47/T10/issues/8#issuecomment-739257424)
//!
//! > TODO 需要更新文档部分

use std::any::{TypeId, type_name};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit, transmute};
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
    Owned              = 0b1011,
    /// 从 Rust 一侧共享过来的对象
    SharedFromHost     = 0b0010,
    /// 从 Rust 一侧可变共享过来的对象
    MutSharedFromHost  = 0b0011,
    /// 这个对象正从 T10 共享到 Rust
    SharedToHost       = 0b1010,
    /// 这个对象正在 T10 可变共享到 Rust
    MutSharedToHost    = 0b1000,
    /// 对象先可变借用自 Rust，然后再次从 T10 可变共享给 Rust，仅适用于 FFI 调用的情形
    MutReSharedToHost  = 0b0000,
    /// 这个对象已经被移动到 Rust 一侧
    MovedToHost        = 0b0100,
    /// 这个对象正要被回收
    Dropped            = 0b1100,
    /// 这不是堆上对象
    TempObject         = 0b0111
}

impl From<u8> for GcInfo {
    #[inline] fn from(src: u8) -> Self {
        unsafe {
            transmute::<u8, GcInfo>(src)
        }
    }
}

pub const GCINFO_OWNED_MASK: u8 = 0b1000;
pub const GCINFO_DROP_MASK:  u8 = 0b0100;
pub const GCINFO_READ_MASK:  u8 = 0b0010;
pub const GCINFO_WRITE_MASK: u8 = 0b0001;

#[repr(C, align(8))]
union WrapperData<T> {
    value: ManuallyDrop<MaybeUninit<T>>,
    ptr: NonNull<T>
}

#[repr(C, align(8))]
pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    gc_info: u8,
    data_offset: u8,
    data: WrapperData<Ta>,
    _phantom: PhantomData<&'a Ts>
}

impl<'a, Ta: 'a, Ts: 'static> Wrapper<'a, Ta, Ts> {
    pub fn owned(data: Ta) -> Self {
        let mut r = Self {
            data: WrapperData {
                value: ManuallyDrop::new(MaybeUninit::new(data))
            },
            data_offset: 0,
            gc_info: GcInfo::Owned as u8,
            _phantom: PhantomData::default()
        };
        r.data_offset = (&r.data as *const _ as *const () as usize - &r as *const _ as *const () as usize) as u8;
        r
    }

    pub fn shared(data: &Ta) -> Self {
        let mut r = Self {
            data: WrapperData {
                ptr: unsafe { NonNull::new_unchecked(data as *const Ta as *mut Ta) }
            },
            data_offset: 0,
            gc_info: GcInfo::SharedFromHost as u8,
            _phantom: PhantomData::default()
        };
        r.data_offset = (&r.data as *const _ as *const () as usize - &r as *const _ as *const () as usize) as u8;
        r
    }

    pub fn mut_shared(data: &mut Ta) -> Self {
        let mut r = Self {
            data: WrapperData {
                ptr: unsafe { NonNull::new_unchecked(data as *mut Ta) }
            },
            data_offset: 0,
            gc_info: GcInfo::MutSharedFromHost as u8,
            _phantom: PhantomData::default()
        };
        r.data_offset = (&r.data as *const _ as *const () as usize - &r as *const _ as *const () as usize) as u8;
        r
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
    fn dyn_type_name(&self) -> String;
    /// 运行时类型检测
    fn dyn_tyck(&self, tyck_info: &TypeCheckInfo) -> bool;
    /// 运行时获取类型检查信息
    fn dyn_tyck_info(&self) -> TypeCheckInfo;

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

    fn dyn_type_name(&self) -> String {
        type_name::<Ta>().to_string()
    }

    fn dyn_tyck(&self, tyck_info: &TypeCheckInfo) -> bool {
        <Void as StaticBase<Ts>>::tyck(tyck_info)
    }

    fn dyn_tyck_info(&self) -> TypeCheckInfo {
        <Void as StaticBase<Ts>>::tyck_info()
    }

    #[cfg(not(debug_assertions))]
    unsafe fn move_out(&mut self, dest: *mut ()) {
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.take_value());
    }

    #[cfg(debug_assertions)]
    unsafe fn move_out_ck(&mut self, dest: *mut (), dest_ty: TypeId) {
        debug_assert_eq!(GcInfo::from(self.gc_info), GcInfo::Owned);
        debug_assert_eq!(dest_ty, TypeId::of::<MaybeUninit<Ts>>());
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.take_value());
    }
}

/// “值类型对象”的类型标记
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ValueType {
    Int     = 0b00000100,
    Float   = 0b00001000,
    Char    = 0b00001100,
    Bool    = 0b00010000,
    AnyType = 0b00010100,
}

impl From<u8> for ValueType {
    fn from(src: u8) -> Self {
        #[cfg(not(debug_assertions))]
        unsafe { transmute(src) }

        #[cfg(debug_assertions)]
        match src {
            0b00000100 => ValueType::Int,
            0b00001000 => ValueType::Float,
            0b00001100 => ValueType::Char,
            0b00010000 => ValueType::Bool,
            0b00010100 => ValueType::AnyType,
            _ => unreachable!("invalid ValueType")
        }
    }
}

pub(crate) const VALUE_MASK      : u8 = 0b00000001;
pub(crate) const CONTAINER_MASK  : u8 = 0b00000010;
pub(crate) const VALUE_TYPE_MASK : u8 = 0b00011100;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ValueTypedDataInner {
    /// 整数
    pub int: i64,
    /// 浮点数
    pub float: f64,
    /// 字符
    pub ch: char,
    /// 布尔
    pub boolean: bool,
    /// 内部表示
    pub repr: u64
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValueTypedData {
    pub tag: usize,
    pub inner: ValueTypedDataInner
}

#[repr(C, align(8))]
pub struct CustomVTable {
    pub dyn_type_id: fn(*const CustomVTable) -> std::any::TypeId,
    pub dyn_type_name: fn(*const CustomVTable) -> String,
    pub dyn_tyck: fn(*const CustomVTable, &TypeCheckInfo) -> bool,
    pub dyn_tyck_info: fn(*const CustomVTable) -> TypeCheckInfo,

    #[cfg(not(debug_assertions))]
    pub move_out: unsafe fn (*mut (), *const CustomVTable, *mut ()),

    #[cfg(debug_assertions)]
    pub move_out_ck: unsafe fn (*mut (), *const CustomVTable, *mut (), std::any::TypeId),
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CustomFatPtr {
    data: *mut (),
    vtable: *const CustomVTable
}

/// 一个通用的“值”
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    /// 堆对象指针
    pub ptr: *mut dyn DynBase,
    /// 堆对象指针的低层表示
    pub ptr_inner: FatPointer,
    /// 值类型数据
    pub value_typed_data: ValueTypedData,
    /// 自定义胖指针，用于处理容器类型
    pub custom_fat_ptr: CustomFatPtr
}

impl Value {
    /// 创建一个空指针或者空值
    #[inline] pub fn null() -> Self {
        Self {
            ptr_inner: FatPointer::null()
        }
    }

    #[inline] pub fn is_null(&self) -> bool {
        unsafe {
            self.ptr_inner.part1 == 0
        }
    }

    #[inline] pub fn is_value(&self) -> bool {
        unsafe {
            self.ptr_inner.part1 as u8 & VALUE_MASK != 0
        }
    }

    #[inline] pub fn is_ptr(&self) -> bool {
        unsafe {
            self.ptr_inner.part1 as u8 & VALUE_MASK == 0
        }
    }

    /// # safety
    /// requires the data to be not null
    #[inline] pub unsafe fn type_id(&self) -> TypeId {
        if self.ptr_inner.part1 as u8 & VALUE_MASK != 0 {
            match ValueType::from(self.ptr_inner.part1 as u8 & VALUE_TYPE_MASK) {
                ValueType::Int => TypeId::of::<i64>(),
                ValueType::Float => TypeId::of::<f64>(),
                ValueType::Char => TypeId::of::<char>(),
                ValueType::Bool => TypeId::of::<bool>(),
                ValueType::AnyType => todo!("What data type should we use for this?")
            }
        } else if self.ptr_inner.part1 as u8 & CONTAINER_MASK != 0 {
            let f = (*self.custom_fat_ptr.vtable).dyn_type_id;
            f(self.custom_fat_ptr.vtable)
        } else {
            self.ptr.as_ref().unwrap_unchecked().dyn_type_id()
        }
    }

    #[inline] pub fn gc_info(&self) -> GcInfo {
        if self.is_value() {
            GcInfo::TempObject
        } else {
            unsafe {
                GcInfo::from(*(self.ptr as *mut u8))
            }
        }
    }

    #[inline] pub unsafe fn set_gc_info(&self, gc_info: GcInfo) {
        if self.is_ptr() {
            *(self.ptr as *mut u8) = gc_info as u8;
        } else {
            // do nothing, does not matter
        }
    }

    #[inline] pub unsafe fn as_ref<'a, T>(&self) -> &'a T {
        // TODO this is nasty
        debug_assert!(self.is_ptr());
        // TODO this offset operation is for 64bit platform only
        if self.gc_info() as u8 & GCINFO_OWNED_MASK != 0 {
            let offset = *(self.ptr as *mut u8).offset(1);
            let r = NonNull::new_unchecked((self.ptr as *mut u8).offset(offset as isize) as *mut T);
            transmute::<&T, &'a T>(r.as_ref())
        } else {
            let offset = *(self.ptr as *mut u8).offset(1);
            let rr = NonNull::new_unchecked((self.ptr as *mut u8).offset(offset as isize) as *mut *mut T);
            let r = NonNull::new_unchecked(*rr.as_ref());
            transmute::<&T, &'a T>(r.as_ref())
        }
    }

    #[inline] pub unsafe fn as_mut<'a, T>(&self) -> &'a mut T {
        // TODO this is nasty
        debug_assert!(self.is_ptr());
        if self.gc_info() as u8 & GCINFO_OWNED_MASK != 0 {
            let offset = *(self.ptr as *mut u8).offset(1);
            let mut mr = NonNull::new_unchecked((self.ptr as *mut u8).offset(offset as isize) as *mut T);
            transmute::<&mut T, &'a mut T>(mr.as_mut())
        } else {
            let offset = *(self.ptr as *mut u8).offset(1);
            let rmr = NonNull::new_unchecked((self.ptr as *mut u8).offset(offset as isize) as *mut *mut T);
            let mut mr = NonNull::new_unchecked(*rmr.as_ref());
            transmute::<&mut T, &'a mut T>(mr.as_mut())
        }
    }
}

impl<'a> From<*mut dyn DynBase> for Value {
    fn from(ptr: *mut dyn DynBase) -> Self {
        Self { ptr }
    }
}

impl<'a> From<i64> for Value {
    fn from(int: i64) -> Self {
        Self {
            value_typed_data: ValueTypedData {
                tag: (ValueType::Int as usize) | (VALUE_MASK as usize),
                inner: ValueTypedDataInner {
                    int
                }
            }
        }
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
    fn from(boolean: bool) -> Self {
        Self {
            value_typed_data: ValueTypedData {
                tag: (ValueType::Bool as usize) | (VALUE_MASK as usize),
                inner: ValueTypedDataInner {
                    boolean
                }
            }
        }
    }
}
