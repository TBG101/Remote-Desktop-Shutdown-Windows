#![windows_subsystem = "windows"]
use std::io::{self};
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    thread::spawn(|| {
        packet_loop();
    });

    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let _ = app.set_icon_from_file("icon.ico");

    _ = app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    });

    add_to_startup().expect("Failed to add to startup");

    _ = app.wait_for_message();
}

fn send_response(socket: &UdpSocket, response: &str, sender: &str) {
    // Send the response to the sender
    socket
        .send_to(response.as_bytes(), sender)
        .expect("Failed to send response");
}

fn execute_cmd(cmd: &str) -> io::Result<String> {
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

fn excute_shutdown_command() -> String {
    let output = Command::new("cmd")
        .args(&["/C", "shutdown /s"])
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&output.stdout);
    return result.to_string();
}

fn packet_loop() {
    let listen_address = "0.0.0.0";
    let listen_port = 8888;

    let udp_socket = UdpSocket::bind(format!("{}:{}", listen_address, listen_port))
        .expect("Failed to bind to address");

    println!(
        "Listening for UDP packets on {}:{}",
        listen_address, listen_port
    );

    let mut buf = [0; 1024];

    loop {
        let (size, sender) = udp_socket
            .recv_from(&mut buf)
            .expect("Failed to receive data");
        let mut message = String::from_utf8_lossy(&buf[0..size]).to_string();

        if message == "shutdown" {
            excute_shutdown_command();
        } else if message.starts_with("cmd") && message.len() > 4 {
            let cmd = message.replace("cmd ", "");
            match execute_cmd(&cmd) {
                Ok(output) => send_response(&udp_socket, &output, &sender.to_string()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

fn add_to_startup() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;

    let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

    run_key.set_value("RemoteShutdown", &exe_path)?;
    Ok(())
}
