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
