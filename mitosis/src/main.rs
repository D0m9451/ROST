use std::env;
use std::fs;
use std::path::Path;


fn mitosis(count: u32) -> std::io::Result<()> {
    let current = env::current_exe()?;
    println!("{:?}", current);

    let filename = format!(".mitosis{}.exe", count);
    let copy = current.with_file_name(filename);
    fs::copy(&current, &copy)?;

    println!("new file made at {:?}", copy);

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let _ = Command::new("attrib")
            .args(&["+H", copy.to_str().unwrap()])
            .output()
            .expect("Failed to set hidden attribute");
        println!("Hidden attribute set on Windows.");
    }
/* 
    Command::new(&copy)
        .spawn()
        .expect("Failed to run new copy");

*/
    Ok(())
}

fn main() {
    let mut count = 0;
    while count < 3 {
        count += 1;
        if let Err(e) = mitosis(count) {
            println!("Error replicating: {}", e);
        }
    }
}