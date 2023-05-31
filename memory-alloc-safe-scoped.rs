fn main() {
    let my_number: i32 = 42; // Allocate an integer on the stack

    let my_boxed_number: Box<i32> = Box::new(my_number); // Create a Box to hold the value

    println!("My number: {}", *my_boxed_number); // Access the value using dereference operator *

    // No explicit deallocation needed. Memory is automatically freed when `my_boxed_number` goes out of scope.
}
