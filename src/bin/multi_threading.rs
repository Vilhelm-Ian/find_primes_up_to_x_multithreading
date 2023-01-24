use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let mut threads = Vec::new();
    let handle = thread::spawn(move || {
        for i in 10..10000000 / 2 {
            is_prime(&i);
        }
    });
    threads.push(handle);
    let handle = thread::spawn(move || {
        for i in 10000000 / 2..10000000 {
            is_prime(&i);
        }
    });
    threads.push(handle);
    for handle in threads {
        handle.join().unwrap()
    }
}

fn is_prime(num: &i32) -> bool {
    let mut i = 3;
    let sqrt = (*num as f32).sqrt() as i32;
    while i <= sqrt {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
