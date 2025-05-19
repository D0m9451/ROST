/*use std::process::Command;
 
fn open_terminal_with_command(command: &str) {
    if cfg!(target_os = "windows") {
        // Windows: use cmd
        Command::new("cmd")
            .args(&["/C", command])
            .spawn()
            .expect("Failed to open terminal on Windows");

    } else if cfg!(target_os = "linux") {
        // Linux: use gnome-terminal
        Command::new("gnome-terminal")
            .arg("--")
            .arg("bash")
            .arg("-c")
            .arg(format!("{}; exec bash", command))
            .spawn()
            .expect("Failed to open terminal on Linux");
*/
use std::process::Command;
use std::env;

fn main() {
    let depth = env::var("DEPTH").unwrap_or_else(|_| "0".to_string())
        .parse::<u32>().unwrap();
    let exe = env::current_exe().expect("could not find exe :(");

    if depth < 3 {
        println!("Spawning 3 processes at depth: {}", depth + 1);

        for _ in 0..3{
            Command::new("cmd")
                .args(&["/K", "start"])
                .arg(&exe)
                .env("DEPTH", (depth + 1) .to_string())
                .spawn()
                .expect("Failed to open terminal window");
        }
        
    }
}
