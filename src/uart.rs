use core::fmt::Write;

// Simple uart driver for NS16550A.
// datasheet: https://datasheetspdf.com/datasheet/NS16550A.html
pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        return Self { base_address };
    }

    pub fn init(&mut self) {
        // Note: ignore the divisor because this driver will just run with qemu
        // Enable uart
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Set the word length bit 0 and word length bit 1
            ptr.add(3).write_volatile(0b11);
            // Enable FIFIO
            ptr.add(2).write_volatile(0b1);
            // Enable receiver buffer interrupt
            ptr.add(1).write_volatile(0b1);
        }
    }

    /// Put a byte to the write buffer.
    pub fn put(&mut self, c: u8) {
        let ptr  = self.base_address as *mut u8;
        unsafe{
            ptr.add(0).write_volatile(c);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr  = self.base_address as *mut u8;
        unsafe {
            // Check if data ready (first bit of LCR set)
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            } else {
                // Both transmitter and receiver buffer are at byte 0
                Some(ptr.add(0).read_volatile())
            }
        }
    }
}

// Implement Write trait for Uart, so we can use it as a rust writer
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            self.put(c);
        }

        Ok(())
    }
}
