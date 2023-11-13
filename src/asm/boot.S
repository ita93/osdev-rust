.option norvc
.section .data

.section .text.init
.global _start
_start:
    // Get the ID of the current running hart.
    // Read the value of mhartid from CSRS to t0
    // https://book.rvemu.app/hardware-components/03-csrs.html
    csrr    t0, mhartid
    // Jump to (3) if mhartid is not 0. It is an infinite loop.
    // purpose: make sure the program will only run on thread 0th. if the thread id is not 0,
    // we will just spin there forever.
    bnez    t0, 3f
    // clear the SATP (supervisor address translation and protection.
    // this will disable logical(?) address translation, so our program will run completely in physical address.
    csrw    satp, zero
    // This option block is to tell compiler that do not relax any instruction in this block
    // Firstly, save all current options (somewhere?).
    .option push
    // Next, change what we want to change
    // .option norelax tells the linker to not relax AUIPC+ADDI to a gp relative reference (constant pool).
    // Obviously the load of gp needs to be fully qualified and can’t be relaxed.
    .option norelax
    // load address (far)
    // _global_pointer is a memory address and we load gp with this address.
        la gp, _global_pointer
    // Restore the stored options
    .option pop

    // Load _bss_start into a0, _bss_end into a1
    // Jump to (2) if a0 is bigger than a1
    // if _bss_start >= _bss_end then goto 2 end
    la  a0, _bss_start
    la  a1, _bss_end
    bgeu    a0, a1, 2f
1:
    // Clear bss section
    // a0 := _bss_start
    // a1 := _bss_end
    // while a0 < a1 do
    //      *a0 = 0
    //      a0 += 8
    // done
    #if __ricsv_xlen == 64
        sd zero, (a0)
    #elif __ricsv_xlen == 32
        sw zero, (a0)
    #endif

    addi    a0, a0, 8
    bltu    a0, a1, 1b
2:
    // setup the stack pointer (sp) address.
    la sp, _stack
    // li means load immediate => load immediate value into register t0
    // set following bits of mstatus CSR
    // MIE(3): enable global interrupt
    // MPIE(7): holds the value of the interrupt-enable bit active prior to the trap.
    // MPP(11-12):  return to M-mode (0b11) when we execute mret
    li t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw mstatus, t0

    // set the program counter of M-mode to kmain,
    // this mean kmain will be called when we call mret.
    la t1, kmain
    csrw mepc, t1

    // set the trap handler address (M-mode) to trap function (in rust code)
    // this function will be called on exception (interrupt)
    la t2, trap
    csrw mtvec, t2

    // mie = machine mode interrupt enable
    // MSIE(3) enable software interrupt
    // MIE(11) enable external interrupts from hardware devices such as ethernet, uart
    li t3, (1 << 3) | (1 << 11)
    csrw mie, t3

    la  ra, 4f
    // return to kmain
    mret
3:
4:
    // Spin the hart
    // Just waiting for an interrupt and if it ever happens, we jump back to (4)
    wfi
    j 4b
