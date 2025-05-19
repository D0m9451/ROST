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

fn main() {
    let mut i = 0;
    while i < 5 {
        Command::new("cmd")
            .args(&["/C", "start", "cmd", "/K", "echo BOOM!"])
            .spawn()
            .expect("Failed to open terminal window");
        i +=1;
    }
}
