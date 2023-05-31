extern crate alloc;

use alloc::alloc::{alloc, dealloc, Layout};

fn main() {
    let layout = Layout::new::<i32>(); // Define the layout for the allocation
    let ptr = unsafe { alloc(layout) }; // Allocate memory

    // Use the allocated memory
    unsafe {
        *(ptr as *mut i32) = 42;
        println!("My number: {}", *(ptr as *mut i32));
    }

    unsafe { dealloc(ptr, layout) }; // Deallocate the memory
}
