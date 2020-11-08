use crate::data::Ptr;

pub trait VMPtrFromRust<'a, T: 'a> {
    unsafe fn from_any(t: T) -> Result<Ptr<'a>, String>;
}