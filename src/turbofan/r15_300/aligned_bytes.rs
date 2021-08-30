use std::ptr::NonNull;
use std::alloc::{Layout, alloc, dealloc};
use std::intrinsics::copy_nonoverlapping;
use std::marker::PhantomData;

struct AlignedBytesRaw {
    ptr: NonNull<u8>,
    cap: usize,
    _phantom: PhantomData<[u8]>
}

impl AlignedBytesRaw {
    pub fn new(cap: usize) -> Self {
        debug_assert!(cap >= 8);
        unsafe {
            let layout = Layout::array::<u8>(cap)
                .unwrap_unchecked()
                .align_to(8)
                .unwrap_unchecked();
            let ptr = alloc(layout);
            let ptr = NonNull::new(ptr).unwrap_unchecked();
            Self {
                ptr,
                cap,
                _phantom: PhantomData::default()
            }
        }
    }

    #[inline] pub unsafe fn extend2(&mut self) {
        let new_cap = self.cap * 2;
        let old_layout = Layout::array::<u8>(self.cap)
            .unwrap_unchecked()
            .align_to(8)
            .unwrap_unchecked();
        let new_layout = Layout::array::<u8>(new_cap)
            .unwrap_unchecked()
            .align_to(8)
            .unwrap_unchecked();
        let old_ptr = self.ptr.as_ptr();
        let new_ptr = alloc(new_layout);
        self.ptr = NonNull::new(new_ptr).unwrap_unchecked();
        self.cap = new_cap;
        copy_nonoverlapping(old_ptr, new_ptr, old_layout.size());
        dealloc(old_ptr, old_layout);
    }
}

impl Drop for AlignedBytesRaw {
    #[inline] fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<u8>(self.cap)
                .unwrap_unchecked()
                .align_to(8)
                .unwrap_unchecked();
            dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

pub struct AlignedBytes {
    raw: AlignedBytesRaw,
    len: usize
}

impl AlignedBytes {
    #[inline] pub fn new() -> Self {
        Self::with_capacity(512)
    }

    #[inline] pub fn with_capacity(cap: usize) -> Self {
        assert!(cap >= 16);
        Self {
            raw: AlignedBytesRaw::new(cap),
            len: 0
        }
    }

    #[inline] pub fn len(&self) -> usize {
        self.len
    }

    #[inline] pub fn push_byte(&mut self, byte: u8) {
        if self.len == self.raw.cap {
            unsafe { self.raw.extend2() }
        }

        unsafe {
            std::ptr::write(self.raw.ptr.as_ptr().offset(self.len as isize), byte);
        }
        self.len += 1;
    }

    #[inline] pub unsafe fn push_zero_bytes(&mut self, count: usize) {
        debug_assert!(count <= 8);
        if self.raw.cap - self.len < count {
            self.raw.extend2();
        }
        std::ptr::write_bytes(self.raw.ptr.as_ptr().offset(self.len as isize), 0, count);
    }

    #[inline] pub fn push_u32(&mut self, dword: u32) {
        if self.raw.cap - self.len < 4 {
            unsafe { self.raw.extend2(); }
        }
        unsafe {
            std::ptr::write(self.raw.ptr.as_ptr().offset(self.len as isize) as *mut u32, dword);
        }
        self.len += 4;
    }

    #[inline] pub fn push_u64(&mut self, qword: u64) {
        if self.raw.cap - self.len < 4 {
            unsafe { self.raw.extend2(); }
        }
        unsafe {
            std::ptr::write(self.raw.ptr.as_ptr().offset(self.len as isize) as *mut u64, qword);
        }
        self.len += 8;
    }

    #[inline] pub unsafe fn write_byte(&mut self, pos: usize, byte: u8) {
        debug_assert!(pos < self.len);
        std::ptr::write(self.raw.ptr.as_ptr().offset(pos as isize), byte);
    }

    #[inline] pub unsafe fn write_u32(&mut self, pos: usize, dword: u32) {
        debug_assert!(pos < self.len);
        debug_assert!(pos + 4 <= self.len);
        std::ptr::write(self.raw.ptr.as_ptr().offset(pos as isize) as *mut u32, dword);
    }

    #[inline] pub unsafe fn write_u64(&mut self, pos: usize, qword: u64) {
        debug_assert!(pos < self.len);
        debug_assert!(pos + 8 <= self.len);
        std::ptr::write(self.raw.ptr.as_ptr().offset(pos as isize) as *mut u64, qword);
    }

    #[inline] pub unsafe fn read_byte(&self, pos: usize) -> u8 {
        debug_assert!(pos < self.len);
        std::ptr::read(self.raw.ptr.as_ptr().offset(pos as isize))
    }

    #[inline] pub unsafe fn read_u32(&self, pos: usize) -> u32 {
        debug_assert!(pos < self.len);
        debug_assert!(pos + 4 <= self.len);
        std::ptr::read(self.raw.ptr.as_ptr().offset(pos as isize) as *mut u32 as *const _)
    }

    #[inline] pub unsafe fn read_u64(&self, pos: usize) -> u64 {
        debug_assert!(pos < self.len);
        debug_assert!(pos + 8 <= self.len);
        std::ptr::read(self.raw.ptr.as_ptr().offset(pos as isize) as *mut u64 as *const _)
    }

    #[inline] pub unsafe fn offset_ptr<T>(&self, offset: usize) -> *const T {
        debug_assert!(offset < self.len);
        self.raw.ptr.as_ptr().offset(offset as isize) as *const u8 as *const _
    }

    #[cfg(debug_assertions)]
    pub fn assert_aligned(&self, alignment: usize) {
        assert_eq!(self.raw.ptr.as_ptr() as usize % 8, 0);
        assert_eq!(self.len % alignment, 0);
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)] pub fn assert_aligned(&self, _alignment: usize) {}
}
