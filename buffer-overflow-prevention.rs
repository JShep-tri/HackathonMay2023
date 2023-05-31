fn main() {
    let mut buffer: [u8; 4] = [0; 4]; // Create a fixed-size buffer of length 4

    // Attempt to write more than the buffer's capacity
    buffer[0] = 1;
    buffer[1] = 2;
    buffer[2] = 3;
    buffer[3] = 4;
    //buffer[4] = 5; // Uncommenting this line would result in a compile-time error
    // But the below code is fine and will result in a runtime error
    //for i in 0..5 {
    //    println!("{:?}", buffer[i]);
    //}

    for i in 0..5 {
        if let Some(value) = buffer.get(i) {
            println!("Found value {} at index {}", value, i);
        } else {
            println!("Index {} is out of bounds", i);
        }
    }
    println!("Buffer content: {:?}", buffer);
}




