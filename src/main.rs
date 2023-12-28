#![windows_subsystem = "windows"]
use std::net::UdpSocket;
use std::process::Command;
use std::thread;

fn main() {
    thread::spawn(|| {
        packet_loop();
    });

    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
    // w.set_tooltip(&"Whatever".to_string());

    let _ = app.set_icon_from_file("icon.ico").unwrap();

    _ = app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    });

    // println!("Waiting on message!");
    _ = app.wait_for_message();
}

fn packet_loop() {
    // Define the address anld port to listen on
    let listen_address = "0.0.0.0";
    let listen_port = 8888;

    // Create a UDP socket
    let udp_socket = UdpSocket::bind(format!("{}:{}", listen_address, listen_port))
        .expect("Failed to bind to address");

    println!(
        "Listening for UDP packets on {}:{}",
        listen_address, listen_port
    );
    // Buffer to store received data
    let mut buf = [0; 1024];
    loop {
        // Receive data and the address of the sender
        let (size, sender) = udp_socket
            .recv_from(&mut buf)
            .expect("Failed to receive data");
        // Convert the received data to a UTF-8 string
        let message = String::from_utf8_lossy(&buf[0..size]);

        // Print the received message and sender's address
        println!("Received message '{}' from {}", message, sender);
        if message == "shutdown" {
            let output = Command::new("cmd")
                .args(&["/C", "shutdown /s"])
                .output()
                .expect("failed to execute process");

            let result = String::from_utf8_lossy(&output.stdout);
            println!("Command output:\n{}", result);
        }
    }
}
