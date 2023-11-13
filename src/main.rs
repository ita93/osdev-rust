#![no_std]
#![no_main]

use core::arch::global_asm;

use core::panic::PanicInfo;

global_asm!(include_str!("asm/boot.S"));

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[export_name = "trap"]
fn trap() {
    loop{}
}

#[no_mangle]
extern "C"
fn kmain() {
	// Main should initialize all sub-systems and get
	// ready to start scheduling. The last thing this
	// should do is start the timer.
}
