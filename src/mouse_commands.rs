use std::{
    io,
    process::{Command, Stdio},
};

#[cfg(target_os = "windows")]
use enigo::{Axis, Button, Direction, Enigo, Mouse};

use uinput::event::controller::Controller::Mouse as Umouse;
use uinput::event::controller::Mouse::Middle;
use uinput::event::controller::Mouse::Right;
use uinput::event::{controller::Mouse::Left, relative::Wheel};

use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;
use uinput::event::relative::Relative::Wheel as relativeWheel;
use uinput::event::Event::{Controller, Relative};

#[cfg(target_os = "windows")]
pub fn mouse_right_down(enigo: &mut Enigo) {
    enigo.button(Button::Right, Direction::Press);
}

#[cfg(target_os = "windows")]
pub fn mouse_right_up(enigo: &mut Enigo) {
    enigo.button(Button::Right, Direction::Release);
}

#[cfg(target_os = "windows")]
pub fn mouse_left_down(enigo: &mut Enigo) {
    enigo.button(Button::Left, Direction::Press);
}

#[cfg(target_os = "windows")]
pub fn mouse_left_up(enigo: &mut Enigo) {
    #[cfg(target_os = "windows")]
    enigo.button(Button::Left, Direction::Release);
}

#[cfg(target_os = "windows")]
pub fn mouse_scroll(enigo: &mut Enigo, message: &str) {
    if let Ok(dy) = message.replace("scroll", "").parse::<i32>() {
        enigo.scroll(dy, Axis::Vertical);
    }
}

#[cfg(target_os = "windows")]
pub fn handle_mouse_move(message: &str, host: &str, enigo: &mut Enigo) {
    let offset_str = message
        .replace(&format!("mouse move {} ", host), "")
        .replace("mouse move all ", "");
    let offsets: Vec<&str> = offset_str.split_whitespace().collect();
    if let (Some(dx_str), Some(dy_str)) = (offsets.get(0), offsets.get(1)) {
        if let (Ok(dx), Ok(dy)) = (dx_str.parse::<f32>(), dy_str.parse::<f32>()) {
            mouse_move(enigo, dx, dy);
        } else {
            eprintln!("Error: Invalid offset values");
        }
    } else {
        eprintln!("Error: Invalid offset format");
    }
}

#[cfg(target_os = "windows")]
pub fn mouse_move(enigo: &mut Enigo, dx: f32, dy: f32) -> bool {
    let dx = (dx * 5.0) as i32;
    let dy = (dy * 5.0) as i32;
    if (dx == 0) && (dy == 0) {
        return false;
    }

    enigo.move_mouse(dx, dy, enigo::Coordinate::Rel);
    return true;
}

#[cfg(target_os = "windows")]
pub fn mouse_left_click(enigo: &mut Enigo) {
    enigo.button(Button::Left, Direction::Click);
}

#[cfg(target_os = "windows")]
pub fn mouse_right_click(enigo: &mut Enigo) {
    enigo.button(Button::Right, Direction::Click);
}

#[cfg(target_os = "windows")]
pub fn mouse_middle_click(enigo: &mut Enigo) {
    #[cfg(target_os = "windows")]
    enigo.button(Button::Middle, Direction::Click);
}

// Linux only
#[cfg(not(target_os = "windows"))]
pub fn create_umouse() -> uinput::Device {
    let device = uinput::default().unwrap().name("uinput-mouse").unwrap();
    device
        .event(Relative(Position(X)))
        .unwrap()
        .event(Relative(Position(Y)))
        .unwrap()
        .event(Relative(relativeWheel(Wheel::Vertical)))
        .unwrap()
        .event(Controller(Umouse(Left)))
        .unwrap()
        .event(Controller(Umouse(Right)))
        .unwrap()
        .event(Controller(Umouse(Middle)))
        .unwrap()
        .create()
        .unwrap()
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_up(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Right)), 0).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_up(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Left)), 0).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_down(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Left)), 1).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_down(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Right)), 1).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_scroll(device: &mut uinput::Device, message: &str) {
    if let Ok(dy) = message.replace("scroll", "").parse::<i32>() {
        device.send(Wheel::Vertical, dy).unwrap();
    }
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_click(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Left)), 1).unwrap();
    device.synchronize().unwrap();
    device.send(Controller(Umouse(Left)), 0).unwrap();
    device.synchronize().unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_click(device: &mut uinput::Device) {
    use uinput::event::Press;

    device.send(Controller(Umouse(Right)), 1).unwrap();
    device.synchronize().unwrap();
    device.send(Controller(Umouse(Right)), 0).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_middle_click(device: &mut uinput::Device) {
    device.send(Controller(Umouse(Middle)), 1).unwrap();
    device.synchronize().unwrap();
    device.send(Controller(Umouse(Middle)), 0).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_move(device: &mut uinput::Device, dx: f32, dy: f32) -> bool {
    let dx = (dx * 5.0) as i32;
    let dy = (dy * 5.0) as i32;
    if (dx == 0) && (dy == 0) {
        return false;
    }

    device.send(X, dx).unwrap();
    device.send(Y, dy).unwrap();
    return true;
}

#[cfg(not(target_os = "windows"))]
pub fn handle_mouse_move(message: &str, host: &str, device: &mut uinput::Device) -> bool {
    let offset_str = message
        .replace(&format!("mouse move {} ", host), "")
        .replace("mouse move all ", "");
    let offsets: Vec<&str> = offset_str.split_whitespace().collect();

    match (offsets.get(0), offsets.get(1)) {
        (Some(dx_str), Some(dy_str)) => {
            if let (Ok(dx), Ok(dy)) = (dx_str.parse::<f32>(), dy_str.parse::<f32>()) {
                return mouse_move(device, dx, dy);
            } else {
                eprintln!("Error: Invalid offset values");
            }
        }
        _ => {
            eprintln!("Error: Invalid offset format");
        }
    }
    return true;
}
