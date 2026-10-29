use winreg::enums::*;
use winreg::RegKey;
use std::io::{self};

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
