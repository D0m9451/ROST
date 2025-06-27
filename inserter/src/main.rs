use std::fs::File;
use text_io::read;
use std::io::{self, Read, Write};
use std::env;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let key: String = String::from("1783oh87fqif7u4yo82uyqufh87yi");


    println!("WARNING: make sure there ar NO SPACES in the file paths you input");
    println!("Enter image file path(like dunk.jpg): ");
    let img: String = read!();

    println!("Enter file path (like funny.exe or c4ingredients.txt): ");
    let file: String = read!();

    println!("Enter output path img file (like final.jpg)");
    let new: String = read!();

    
    let mut imgf = File::open(&img)
        .expect("Failed to open img file");
    let mut idata = Vec::new();
    imgf.read_to_end(&mut idata)
        .expect("failed to read img data");
    

    let mut filef = File::open(&file)
        .expect("Failed to open inserter file");
    let mut fdata = Vec::new();
    filef.read_to_end(&mut fdata)
        .expect("Failed to read inserter file data");
    

    let mut hidden = File::create(&new)
        .expect("Failed to create new file");

    hidden.write_all(&idata)
        .expect("Failed to write img data to file");


    hidden.write_all(key.as_bytes())
        .expect("Failed to insert key");
    

    hidden.write_all(&fdata)
        .expect("Failed to write file data to file");

    println!("Succsufully inserted {} into {} to {}", img, file, new);
    sleep(Duration::from_secs(5))
}
