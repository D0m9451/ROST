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

fn ifelse(x: f64, _y: f64) {
    if x %2.0 == 0.0 {
        println!("{} is a even number!", x)
    }
    else {
        println!("{} is an odd number!", x)
    }
}