fn main() {
    println!("Hello, world yegggo!");
    count();
}

fn count() {
    let mut x: f64 = 0.0;
    let y: f64 = 99.0;

    while x < y {
        x += 1.0;
        println!("{}", x);
    }
    ifelse(x, y);
}

fn ifelse(x: f64, y: f64) {
    let mut count = 0;
    while true {
        if x < 0.0 {
            let mut x = y*x;
            println!("{}", x);
        }
        else {
            let mut x = 3.185*(y*x);
        }
        count += 1;
        if x < 0.0 {
            break;
        }
    }
}