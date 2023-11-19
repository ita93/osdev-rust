use core::arch::asm;

// CSR utilities
// CSR are 32bit registers
pub fn read(addr: u16) -> u32 {
    let result: u32 = 0;
    unsafe {
        asm!("csrrs {0}, {1}, {2}",
             inout(reg)result => _,
             in(reg) addr,
             in(reg) 0,
        );
    }

    result
}

pub trait CSR {
    fn to_u32(&self) -> u32;
    fn addr(&self) -> u16;
    fn update(&mut self, val: u32);

    fn read_and_write(&mut self) {
        let addr = self.addr();
        let val = self.to_u32();
        let result: u32 = 0;

        unsafe {
            asm!("csrrs {0}, {1}, {2}", inout(reg) result => _,
                in(reg) addr, in(reg)val);
        }
    }

    fn bit_set(&self, bitvec: u32) {
        unsafe {
            asm!("csrrw x0, {0}, {1}", in(reg)self.addr(), in(reg)bitvec);
        }
    }

    fn bit_clear(&self, bitvec: u32) {
        unsafe {
            asm!("csrrc 0x, {0}, {1}", in(reg)self.addr(), in(reg)bitvec);
        }
    }
}

const SATP_ADDR: u16 = 0x180;
struct SATP {
    paging_on: bool,
    ppn: u32, //physical page number
}

impl SATP {
    pub fn new(ppn: u32, paging_on: bool) -> SATP {
        SATP { ppn, paging_on }
    }

    pub fn read() -> SATP {
        let v = read(SATP_ADDR);
        SATP {
            paging_on: bit_range(v, 31, 32) == 1,
            ppn: bit_range(v, 0, 22),
        }
    }
}

impl CSR for SATP {
    fn to_u32(&self) -> u32 {
        let paging_on = if self.paging_on { 1 } else { 0 };
        (paging_on << 31) | self.ppn
    }

    fn addr(&self) -> u16 {
        SATP_ADDR
    }

    fn update(&mut self, val: u32) {
        self.paging_on = bit_range(val, 31, 32) == 1;
        self.ppn = bit_range(val, 0, 22);
    }
}

pub fn bit_range(mut value: u32, lb: u8, ub: u8) -> u32 {
    value <<= 32 - lb;
    value >>= 31 - (lb - ub);
    value
}
