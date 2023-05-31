fn main() {
    let mut vec: Vec<i32> = Vec::new();  // Create an empty vector

    vec.push(1);  // Add elements to the vector
    vec.push(2);
    vec.push(3);

    println!("Vector: {:?}", vec);  // Print the vector

    // Add more elements using a loop
    for i in 4..=6 {
        vec.push(i);
    }

    println!("Updated Vector: {:?}", vec);  // Print the updated vector
    println!("vec[6]: {:?}", vec[6]);
    
}
