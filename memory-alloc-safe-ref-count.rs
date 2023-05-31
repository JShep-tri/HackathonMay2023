use std::rc::Rc;

// Define a simple struct
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create a shared reference-counted pointer to a Person
    let person = Rc::new(Person {
        name: "Alice".to_string(),
        age: 25,
    });

    // Clone the Rc to increase the reference count
    let person_clone1 = Rc::clone(&person);
    let person_clone2 = Rc::clone(&person);

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Reference Count: {}", Rc::strong_count(&person));
}
