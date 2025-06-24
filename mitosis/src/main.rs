use std::env;
use std::fs;
use std::path::Path;


fn mitosis() -> std::io::Result<()> {
    let current = env::current_exe()?;
    println!("{:?}", current);

    let copy = current.with_file_name(".mitosis.exe");
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

    Ok(())
}

fn main() {
    if let Err(e) = mitosis() {
        println!("Error replicating: {}", e);
    }
}