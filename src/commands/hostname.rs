use std::process::Command;

pub fn get_hostname() -> Result<String, std::io::Error> {
    let output = Command::new("hostname").output()?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string())
}