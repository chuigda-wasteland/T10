use crate::data::Ptr;

// We cannot use a single pointer since we'd like to support value types.
// `Ptr` always use heap allocation!

pub trait VMPtrFromRust<'a, T: 'a> {
    unsafe fn from_any(t: T) -> Result<Ptr<'a>, String>;
}