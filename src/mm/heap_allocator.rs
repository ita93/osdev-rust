use buddy_system_allocator::LockedHeap;

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

pub const KERNEL_HEAP_SIZE: usize = 100 * 1024 * 1024;

pub fn print_heap_info() {
    unsafe {
        println!(
            "Heap informat: start at {}, size: {} bytes\n",
            HEAP_START, HEAP_SIZE
        );
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_SPACE.as_ptr() as usize, HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
