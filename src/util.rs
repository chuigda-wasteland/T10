#[repr(C)]
#[derive(Copy, Clone)]
pub struct FatPointer {
    pub part1: usize,
    pub part2: usize
}

impl FatPointer {
    pub fn from_parts(part1: usize, part2: usize) -> Self {
        Self { part1, part2 }
    }

    pub fn null() -> Self {
        Self::from_parts(0, 0)
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::mem::{align_of, size_of};

    use crate::util::FatPointer;

    #[test]
    fn test_usize_capacity() {
        assert!(size_of::<usize>() >= 2);
    }

    #[test]
    fn test_fat_pointer_size() {
        assert_eq!(size_of::<FatPointer>(), size_of::<*mut dyn Any>());
        assert_eq!(size_of::<FatPointer>(), size_of::<&[u8]>());
    }

    #[test]
    fn test_fat_pointer_align() {
        assert!(align_of::<FatPointer>() >= align_of::<*mut dyn Any>());
        assert!(align_of::<FatPointer>() >= align_of::<&[u8]>());
    }
}
