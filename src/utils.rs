use std::io;
use std::process::Command;
use std::process::Stdio;
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(windows)]
pub fn add_to_startup() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;

    let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

    run_key.set_value("RemoteShutdown", &exe_path)?;
    Ok(())
}

#[cfg(not(windows))]
pub fn add_to_startup() -> io::Result<()> {
    Ok(())
}

#[cfg(windows)]
pub fn execute_cmd(cmd: &str) -> Result<String, io::Error> {
    let output = Command::new("cmd")
        .args(["/C", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
#[cfg(not(windows))]
pub fn execute_cmd(cmd: &str) -> Result<String, io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

#[cfg(windows)]
pub fn execute_shutdown_command() -> Result<String, io::Error> {
    let output = Command::new("cmd").args(&["/C", "shutdown /s"]).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(not(windows))]
pub fn execute_shutdown_command() -> Result<String, io::Error> {
    let output = Command::new("shutdown").args(&["-h", "now"]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
