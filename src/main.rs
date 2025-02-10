#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod trayBuilder;
use enigo::{Enigo, MouseControllable};
use std::io::{self};
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

fn main() {
    let handle = thread::spawn(|| {
        packet_loop();
    });

    add_to_startup().expect("Failed to add to startup");
    trayBuilder::build_tray();
    handle.join().expect("Failed to join thread");
}

fn send_response(socket: &UdpSocket, response: &str, sender: &str) {
    let sending_ip = sender.split(":").collect::<Vec<&str>>()[0];
    let sender = format!("{}:{}", sending_ip, 8887);
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
    let binding = execute_cmd("hostname").expect("Failed to get hostname");
    let hostname = binding.trim();

    let cmd_all = "cmd all ";
    let cmd_hostname = format!("cmd {} ", hostname);

    let mouse_move_host = format!("mouse move {} ", hostname);
    let mouse_left_click_host = format!("mouse left click {}", hostname);
    let mouse_right_click_host = format!("mouse right click {}", hostname);

    let mut enigo = Enigo::new();

    let mut buf = [0; 1024];
    loop {
        let (size, sender) = udp_socket
            .recv_from(&mut buf)
            .expect("Failed to receive data");
        let message = String::from_utf8_lossy(&buf[0..size]).to_string();
        println!("Received message: {}", message);

        match message.as_str() {
            msg if msg == format!("shutdown {}", hostname) || msg == "shutdown all" => {
                excute_shutdown_command();
            }
            msg if msg.starts_with(&cmd_hostname) => {
                let cmd = msg.replace(&format!("cmd {} ", hostname), "");
                match execute_cmd(&cmd) {
                    Ok(output) => {
                        let res = format!("cmd {}", output);
                        send_response(&udp_socket, &res, &sender.to_string())
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            msg if msg.starts_with(cmd_all) && msg.len() > 4 => {
                let cmd = msg.replace("cmd all ", "");
                match execute_cmd(&cmd) {
                    Ok(output) => {
                        let res = format!("cmd {}", output);
                        send_response(&udp_socket, &res, &sender.to_string())
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }

            msg if msg.starts_with("get_device") => {
                send_response(
                    &udp_socket,
                    &format!("device: {}", &hostname),
                    &sender.to_string(),
                );
            }
            msg if msg.starts_with("mouse move all") || msg.starts_with(&mouse_move_host) => {
                println!("Handling mouse move");
                println!("Message: {}", message);
                handle_mouse_move(&message, &hostname, &mut enigo);
            }

            msg if msg.starts_with("mouse all")
                || msg.starts_with(&format!("mouse {}", hostname)) =>
            {
                let msg = msg.replace(&format!("mouse {}", hostname), "");
                let msg = msg.replace("mouse all", "");
                let msg = msg.trim();
                println!("Mouse command: {}", msg);
                match msg {
                    "left click" => mouse_left_click(&mut enigo),
                    "right click" => mouse_right_click(&mut enigo, enigo::MouseButton::Right),
                    "middle click" => mouse_right_click(&mut enigo, enigo::MouseButton::Middle),
                    "right click down" => mouse_right_down(&mut enigo),
                    "right click up" => mouse_right_up(&mut enigo),
                    "left click down" => mouse_left_down(&mut enigo),
                    "left click up" => mouse_left_up(&mut enigo),
                    cmd if cmd.starts_with("scroll") => scroll(&mut enigo, msg),

                    _ => eprintln!("Unknown command: {}", message),
                }
            }
            _ => eprintln!("Unknown command: {}", message),
        }
    }
}

fn mouse_right_down(enigo: &mut Enigo) {
    enigo.mouse_down(enigo::MouseButton::Right);
}

fn mouse_right_up(enigo: &mut Enigo) {
    enigo.mouse_up(enigo::MouseButton::Right);
}

fn mouse_left_down(enigo: &mut Enigo) {
    enigo.mouse_down(enigo::MouseButton::Left);
}

fn mouse_left_up(enigo: &mut Enigo) {
    enigo.mouse_up(enigo::MouseButton::Left);
}

fn scroll(enigo: &mut Enigo, message: &str) {
    let dy = match message.replace("scroll", "").trim().parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Invalid scroll value");
            return;
        }
    };
    enigo.mouse_scroll_y(dy);
}

fn handle_mouse_move(message: &str, host: &str, enigo: &mut Enigo) {
    let offset_str = message
        .replace("mouse move all ", "")
        .replace(&format!("mouse move {} ", host), "");
    println!("Offset: {}", offset_str);
    let offset = offset_str.trim().split(" ").collect::<Vec<&str>>();
    if offset.len() != 2 {
        eprintln!("Error: Invalid offset format");
        return;
    }

    let dx = match offset[0].parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Invalid dx value");
            return;
        }
    };

    let dy = match offset[1].parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Invalid dy value");
            return;
        }
    };

    mouse_move(enigo, dx, dy);
}

fn mouse_move(enigo: &mut Enigo, dx: f32, dy: f32) {
    let dx = dx * 10.0;
    let dy = dy * 10.0;
    let dx = dx as i32;
    let dy = dy as i32;

    enigo.mouse_move_relative(dx, dy);
}

fn mouse_left_click(enigo: &mut Enigo) {
    enigo.mouse_click(enigo::MouseButton::Left);
}

fn mouse_right_click(enigo: &mut Enigo, button: enigo::MouseButton) {
    enigo.mouse_click(button);
}

#[cfg(windows)]
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

#[cfg(not(windows))]
fn add_to_startup() -> io::Result<()> {
    Ok(())
}
