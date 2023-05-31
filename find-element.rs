fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &element) in arr.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let target = 3;

    match find_element(&array, target) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found."),
    }
}
