use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    thread::spawn(move || {
        data[0] += 1;
    });

    data[0] += 1;
}
