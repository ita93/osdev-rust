
const PA_WIDTH_SV39: usize = 56;
const PAGE_SIZE_BITS: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        // This will only set last 56 bits of value
        Self(value & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39)-1))
    }
}

impl From<PhysAddr> for usize {
    fn from(value: PhysAddr) -> Self {
       value.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(value: PhysPageNum) -> Self {
        value.0
    }
}

impl PhysAddr {
    pub fn page_offset(&self) -> usize{self.0 & (PAGE_SIZE  - 1)}
    pub fn floor(&self) -> PhysPageNum {PhysPageNum(self.0 / PAGE_SIZE)}
    pub fn ceil(&self) -> PhysPageNum {PhysPageNum((self.0 + PAGE_SIZE - 1)/PAGE_SIZE)}
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor() 
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self{Self(v.0 << PAGE_SIZE_BITS)}
}
