#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use core::panic::PanicInfo;

global_asm!(include_str!("asm/boot.S"));

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println {
    () => ({
       print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!("{}:{} : {}", p.file(), p.line(), info.message().unwrap());
    } else {
        println!("no information given")
    }

    loop {}
}

#[export_name = "trap"]
fn trap() {
    loop {}
}

#[no_mangle]
extern "C" fn kmain() {
    let mut ricv_uart = uart::Uart::new(0x1000_0000);
    // We init the uart here, so every call to uart (even on another object) can
    // do transmit/receive data because the address is global to whole program.
    ricv_uart.init();

    println!("Rust on RISCV");
    println!(
        r"
 ______               __        _______ _______      ______ _______ _______ ______ ___ ___ 
|   __ \.--.--.-----.|  |_     |       |    |  |    |   __ \_     _|     __|      |   |   |
|      <|  |  |__ --||   _|    |   -   |       |    |      <_|   |_|__     |   ---|   |   |
|___|__||_____|_____||____|    |_______|__|____|    |___|__|_______|_______|______|\_____/ 
                                                                                           
"
    );

    // Reading input from Uart
    loop {
        if let Some(c) = ricv_uart.get() {
            match c {
                8 => {
                    // This is a backspace, so we essentially have
                    // to write a space and backup again:
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                _ => {
                    print!("{}", c as char);
                }
            }
        }
    }
}

pub mod uart;
