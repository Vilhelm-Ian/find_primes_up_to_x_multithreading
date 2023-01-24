use std::sync::mpsc::channel;
use std::thread;

fn main() {
    for i in 10..10000000 {
        is_prime(&i);
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
