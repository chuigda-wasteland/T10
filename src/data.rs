use std::any::{TypeId, type_name};
use std::marker::PhantomData;
use std::mem::{MaybeUninit, ManuallyDrop};
use std::ptr::null_mut;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;

use crate::tyck::base::StaticBase;
use crate::tyck::TypeCheckInfo;
use crate::void::Void;

pub enum GcInfo {
    Owned = 0,
    SharedWithHost = 1,
    MutSharedWithHost = 2,
    MovedToHost = 3,
    Dropped = 4,
    Null = 5
}

impl GcInfo {
    pub fn from_u8(src: u8) -> GcInfo {
        match src {
            0 => GcInfo::Owned,
            1 => GcInfo::SharedWithHost,
            2 => GcInfo::MutSharedWithHost,
            3 => GcInfo::MovedToHost,
            4 => GcInfo::Dropped,
            5 => GcInfo::Null,
            _ => unreachable!()
        }
    }
}

union WrapperData<T> {
    value: ManuallyDrop<MaybeUninit<T>>,
    ptr: *mut T
}

impl<T> WrapperData<T> {
    pub unsafe fn borrow_value(&mut self) -> *mut T {
        self.value.as_mut_ptr()
    }

    pub unsafe fn borrow_ptr(&self) -> *mut T {
        self.ptr
    }

    pub unsafe fn take_value(&mut self) -> T {
        ManuallyDrop::take(&mut self.value).assume_init()
    }
}

pub struct Wrapper<'a, Ta: 'a, Ts: 'static> {
    data: WrapperData<Ta>,
    gc_info: AtomicU8,
    _phantom: PhantomData<&'a Ts>
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

pub trait DynBase {
    fn dyn_type_id(&self) -> std::any::TypeId;
    fn dyn_type_name(&self) -> &'static str;
    fn dyn_tyck(&self, tyck_info: &TypeCheckInfo) -> bool;
    fn dyn_tyck_info(&self) -> TypeCheckInfo;

    // TODO differ between `as_ptr_borrow` and `as_ptr_take`
    unsafe fn as_ptr(&mut self) -> *mut ();

    #[cfg(not(debug_assertions))]
    unsafe fn take_out(&mut self, dest: *mut ());

    #[cfg(debug_assertions)]
    unsafe fn take_out(&mut self, dest: *mut (), dest_ty: TypeId);
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

    unsafe fn as_ptr(&mut self) -> *mut () {
        match GcInfo::from_u8(self.gc_info.load(SeqCst)) {
            GcInfo::SharedWithHost => self.data.borrow_ptr() as *mut (),
            GcInfo::MutSharedWithHost => self.data.borrow_ptr() as *mut (),
            GcInfo::Owned => self.data.borrow_value() as *mut (),
            GcInfo::MovedToHost => unreachable!("cannot use moved value"),
            GcInfo::Dropped => unreachable!("cannot use dropped value"),
            GcInfo::Null => null_mut()
        }
    }

    #[cfg(not(debug_assertions))]
    unsafe fn take_out(&mut self, dest: *mut ()) {
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.data.take_value());
    }

    #[cfg(debug_assertions)]
    unsafe fn take_out(&mut self, dest: *mut (), dest_ty: TypeId) {
        debug_assert_eq!(dest_ty, TypeId::of::<MaybeUninit<Ts>>());
        let dest = (dest as *mut MaybeUninit<Ta>).as_mut().unwrap();
        dest.write(self.data.take_value());
    }
}

/*

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::marker::PhantomData;

use crate::tyck::{StaticBase, TypeCheckInfo};
use crate::cast::RustLifetime;

// 5 status = 3bit

pub enum ValueType {
    Int = 0,
    Float = 1,
    Char = 2,
    Byte = 3,
    Bool = 4,
    AnyType = 5,
}

impl ValueType {
    pub fn from_u8(src: u8) -> ValueType {
        match src {
            0 => ValueType::Int,
            1 => ValueType::Float,
            2 => ValueType::Char,
            3 => ValueType::Byte,
            4 => ValueType::Bool,
            _ => unreachable!()
        }
    }
}

// impl !DynBase for &T {}
// impl !DynBase for &mut T {}

impl<T: 'static> DynBase for T {
    fn dyn_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn dyn_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn dyn_type_check(&self, tyck_info: &TypeCheckInfo) -> Result<(), String> {
        <T as StaticBase>::type_check(tyck_info)
    }

    fn dyn_tyck_info(&self) -> TypeCheckInfo {
        <T as StaticBase>::tyck_info()
    }

    fn dyn_lifetime_info(&self) -> RustLifetime {
        <Self as StaticBase>::lifetime_info()
    }
}

pub struct Ptr<'a> {
    pub gc_info: *mut AtomicU8,
    pub data: *mut dyn DynBase,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Ptr<'a> {
    pub fn moved(data: (impl DynBase + 'static)) -> Self {
        Self {
            gc_info: Box::leak(Box::new(AtomicU8::new(GcInfo::OnVMHeap as u8))) as *mut AtomicU8,
            data: Box::leak(Box::new(data)) as *mut dyn DynBase,
            _phantom: PhantomData::default()
        }
    }

    pub fn borrow(data: &'a (impl DynBase + 'static)) -> Self {
        Self {
            gc_info:
                Box::leak(Box::new(AtomicU8::new(GcInfo::SharedWithHost as u8))) as *mut AtomicU8,
            data: data as *const dyn DynBase as *mut dyn DynBase,
            _phantom: PhantomData::default()
        }
    }

    pub fn mut_borrow(data: &'a mut (impl DynBase + 'static)) -> Self {
         Self {
             gc_info: Box::leak(
                 Box::new(
                     AtomicU8::new(GcInfo::MutSharedWithHost as u8))) as *mut AtomicU8,
             data: data as *mut dyn DynBase,
             _phantom: PhantomData::default()
         }
    }

    pub fn gc_info(&self) -> GcInfo {
        unsafe {
            if let Some(info) = self.gc_info.as_ref() {
                GcInfo::from_u8(info.load(SeqCst))
            } else {
                GcInfo::Dropped
            }
        }
    }
}

impl<'a> Clone for Ptr<'a> {
    fn clone(&self) -> Self {
        Self {
            gc_info: self.gc_info,
            data: self.data,
            _phantom: self._phantom,
        }
    }
}

unsafe impl<'a> Send for Ptr<'a> {}
unsafe impl<'a> Sync for Ptr<'a> {}

// For the union!
#[repr(C)]
pub union ValueData {
    pub(crate) ptr: *mut dyn DynBase,
    pub(crate) int: i64,
    pub(crate) float: f64,
    pub(crate) ch: char,
    pub(crate) byte: u8,
    pub(crate) boolean: bool
}

#[repr(C)]
pub struct Value<'a> {
    pub(crate) data: ValueData,
    tag: u8,
    _phantom: PhantomData<&'a ()>
}

impl<'a> Value<'a> {
    fn new(data: ValueData, tag: u8) -> Self {
        Self {
            data,
            tag,
            _phantom: PhantomData::default()
        }
    }

    pub fn from_ptr(ptr: Ptr<'a>) -> Self {
        Self::new(
            ValueData { ptr: ptr.data },
            unsafe { ptr.gc_info.as_ref() }.map_or(
                GcInfo::Null as u8,
                |r| r.load(SeqCst)
            ) | 0x80
        )
    }

    pub fn null_value_type(ty: ValueType) -> Self {
        Self::new(ValueData { int: 0 }, (ty as u8) | 0x40)
    }

    pub fn from_i64(data: i64) -> Self {
        Self::new(ValueData { int: data }, ValueType::Int as u8)
    }

    pub fn from_f64(data: f64) -> Self {
        Self::new(ValueData { float: data }, ValueType::Float as u8)
    }

    pub fn from_char(data: char) -> Self {
        Self::new(ValueData { ch: data }, ValueType::Char as u8)
    }

    pub fn from_u8(data: u8) -> Self {
        Self::new(ValueData { byte: data }, ValueType::Byte as u8)
    }

    pub fn from_bool(data: bool) -> Self {
        Self::new(ValueData { boolean: data }, ValueType::Bool as u8)
    }

    pub fn is_ptr(&self) -> bool {
        (self.tag & 0x80) != 0
    }

    pub fn is_value(&self) -> bool {
        !self.is_ptr()
    }

    pub fn is_null(&self) -> bool {
        (self.is_ptr() && (self.tag & 0x07 == GcInfo::Null as u8))
        || (self.is_value() && (self.tag & 0x40 == 1))
    }

    pub fn type_id(&self) -> std::any::TypeId {
        debug_assert!(!self.is_null());
        if self.is_ptr() {
            debug_assert_ne!(self.tag & 0x7F, GcInfo::Dropped as u8);
            unsafe { self.data.ptr.as_ref() }.unwrap().dyn_type_id()
        } else {
            use std::any::TypeId;
            match ValueType::from_u8(self.tag & 0x07) {
                ValueType::Int => TypeId::of::<i64>(),
                ValueType::Float => TypeId::of::<f64>(),
                ValueType::Char => TypeId::of::<char>(),
                ValueType::Bool => TypeId::of::<bool>(),
                ValueType::Byte => TypeId::of::<u8>(),
                ValueType::AnyType => TypeId::of::<dyn std::any::Any>()
            }
        }
    }
}

 */