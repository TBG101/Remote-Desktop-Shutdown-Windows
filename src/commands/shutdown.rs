use std::process::Command;

pub fn shutdown() {
    let output = Command::new("cmd")
        .args(&["/C", "shutdown /s"])
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&output.stdout);
}
