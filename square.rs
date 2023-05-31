fn square(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value * value),
        None => None,
    }
}

fn main() {
    let input = Some(5);
    let result = square(input);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("No value provided"),
    }
}
