mod mouse_commands;
mod tray_builder;
mod linux_mouse;
mod utils;
use mouse_commands::*;
use std::io::Result;
use std::net::UdpSocket;
use std::thread;
use utils::execute_cmd;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

const LISTEN_ADDRESS: &str = "0.0.0.0";
const LISTEN_PORT: u16 = 8888;
const RESPONSE_PORT: u16 = 8887;

fn main() -> Result<()> {
    let handle = thread::spawn(packet_loop);

    utils::add_to_startup()?;
    tray_builder::build_tray();
    let _ = handle.join().expect("Failed to join thread");
    Ok(())
}

fn send_response(socket: &UdpSocket, response: &str, sender: &str) -> Result<()> {
    let sending_ip = sender.split(':').next().unwrap_or("");
    let sender = format!("{}:{}", sending_ip, RESPONSE_PORT);
    socket.send_to(response.as_bytes(), &sender)?;
    Ok(())
}

fn get_hostname() -> String {
    #[cfg(target_os = "windows")]
    let hostname = execute_cmd("hostname").unwrap();
    #[cfg(not(target_os = "windows"))]
    let hostname = execute_cmd("whoami").unwrap();
    hostname.trim().to_string()
}

fn packet_loop() -> Result<()> {
    let udp_socket = UdpSocket::bind(format!("{}:{}", LISTEN_ADDRESS, LISTEN_PORT))?;
    println!(
        "Listening for UDP packets on {}:{}",
        LISTEN_ADDRESS, LISTEN_PORT
    );

    let hostname = get_hostname();

    let cmd_all = "cmd all ";
    let cmd_hostname = format!("cmd {} ", hostname);
    let mouse_move_host = format!("mouse move {} ", hostname);

    #[cfg(target_os = "windows")]
    let mut device: Enigo = Enigo::new(&Settings::default()).expect("Failed to create enigo");

    let mut buf = [0; 1024];

    #[cfg(not(target_os = "windows"))]
    let mut device = create_umouse();

    loop {
        let (size, sender) = match udp_socket.recv_from(&mut buf) {
            Ok((s, sender)) => (s, sender),
            Err(_) => continue,
        };

        let message = match std::str::from_utf8(&buf[..size]) {
            Ok(m) => m,
            Err(_) => continue,
        };

        match message {
            msg if msg == format!("shutdown {}", hostname) || msg == "shutdown all" => {
                utils::execute_shutdown_command().unwrap();
            }

            msg if msg.starts_with(&cmd_hostname) => {
                let cmd = msg.strip_prefix(&cmd_hostname).unwrap_or("").to_string();
                match execute_cmd(&cmd) {
                    Ok(output) => {
                        send_response(&udp_socket, &format!("cmd {}", output), &sender.to_string())?
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            msg if msg.starts_with(cmd_all) => {
                let cmd = msg.strip_prefix(cmd_all).unwrap_or("").to_string();
                match execute_cmd(&cmd) {
                    Ok(output) => {
                        send_response(&udp_socket, &format!("cmd {}", output), &sender.to_string())?
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            msg if msg.starts_with("get_device") => {
                send_response(
                    &udp_socket,
                    &format!("device: {}", hostname),
                    &sender.to_string(),
                )?;
            }
            msg if msg.starts_with("mouse move all") || msg.starts_with(&mouse_move_host) => {
                if !handle_mouse_move(&message, &hostname, &mut device){
                    continue;
                }
            }
            msg if msg.starts_with("mouse all")
                || msg.starts_with(&format!("mouse {}", hostname)) =>
            {
                handle_mouse_command(&msg, &hostname, &mut device);
            }
            _ => println!("Unknown command: {}", message),
        }
        #[cfg(not(target_os = "windows"))]
        device.synchronize().unwrap();
    }
}

#[cfg(target_os = "windows")]
fn handle_mouse_command(msg: &str, hostname: &str, enigo: &mut Enigo) {
    let msg = msg
        .replace(&format!("mouse {}", hostname), "")
        .replace("mouse all", "")
        .trim()
        .to_string();
    match msg.as_str() {
        "left click" => mouse_left_click(enigo),
        "right click" => mouse_right_click(enigo),
        "middle click" => mouse_middle_click(enigo),
        "right click down" => mouse_right_down(enigo),
        "right click up" => mouse_right_up(enigo),
        "left click down" => mouse_left_down(enigo),
        "left click up" => mouse_left_up(enigo),
        cmd if cmd.starts_with("scroll") => mouse_scroll(enigo, &msg),
        _ => println!("Unknown command: {}", msg),
    }
}

#[cfg(not(target_os = "windows"))]
fn handle_mouse_command(msg: &str, hostname: &str, device: &mut uinput::device::Device) {
    let msg = msg
        .replace(&format!("mouse {}", hostname), "")
        .replace("mouse all", "")
        .trim()
        .to_string();
    match msg.as_str() {
        "left click" => mouse_left_click(device),
        "right click" => mouse_right_click(device),
        "middle click" => mouse_middle_click(device),
        "right click down" => mouse_right_down(device),
        "right click up" => mouse_right_up(device),
        "left click down" => mouse_left_down(device),
        "left click up" => mouse_left_up(device),
        cmd if cmd.starts_with("scroll") => mouse_scroll(device, &msg),
        _ => println!("Unknown command: {}", msg),
    }
}
