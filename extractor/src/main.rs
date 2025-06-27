use std::fs::File;
use std::io::{Read, Write};
use text_io::read;

fn main() {
    let key = b"1783oh87fqif7u4yo82uyqufh87yi";

    println!("What file would you like to extract from?");
    let path: String = read!();

    println!("Enter extracted file name:");
    let efile: String = read!();

    let mut file = File::open(&path)
        .expect("Could not open file path");
    let mut edata = Vec::new();
    file.read_to_end(&mut edata)
        .expect("Could not read file data");

    let vault = edata
        .windows(key.len())
        .position(|window| window == key);

    match vault {
        Some(pos) => {
            let vstart = pos + key.len();
            let vdata = &edata[vstart..];

            let mut rfile = File::create(&efile)
                .expect("Failed to create extracted file");
            rfile.write_all(vdata)
                .expect("Failed to write extracted data to file");

            println!(
                "Successfully extracted data from {} to {}",
                path, efile
            );
        }
        None => {
            println!("Key not found/incorrect in {}", path);
        }
    }
}
