use std::thread;

fn fork(depth: u32) {
    println!("THREADING!!! at depth {}", depth);
    
    if depth < 5 {
        for _ in 0..2 {
            let d = depth + 1;
            thread::spawn(move || {
                fork(d);
            });
        }
    }

    loop {}
}

fn main() {
    fork(0);
}
