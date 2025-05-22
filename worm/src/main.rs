/*use std::net;
use std::thread;*/


fn worm(depth: u32) {
    if depth < 2 {
        println!("{}", depth)
    }
}




fn main() {
    worm(1);
}
