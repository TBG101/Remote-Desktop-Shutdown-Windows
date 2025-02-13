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

use crate::linux_mouse::{self, VirtualMouse};
use evdev::Key;

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

// Linux only
#[cfg(not(target_os = "windows"))]
pub fn create_umouse() -> VirtualMouse {
    VirtualMouse::new().unwrap()
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_up(mouse: &mut VirtualMouse) {
    mouse.button_up(Key::BTN_RIGHT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_up(mouse: &mut VirtualMouse) {
    mouse.button_up(Key::BTN_LEFT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_down(mouse: &mut VirtualMouse) {
    mouse.button_down(Key::BTN_LEFT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_down(mouse: &mut VirtualMouse) {
    mouse.button_down(Key::BTN_RIGHT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_scroll(mouse: &mut VirtualMouse, message: &str) {
    println!("scrolling {}", message);
    if let Ok(dy) = message.replace("scroll", "").parse::<i32>() {
        mouse.scroll(dy).unwrap();
    }
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_left_click(mouse: &mut VirtualMouse) {
    mouse.button_click(Key::BTN_LEFT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_right_click(mouse: &mut VirtualMouse) {
    mouse.button_click(Key::BTN_RIGHT).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_middle_click(mouse: &mut VirtualMouse) {
    mouse.button_click(Key::BTN_MIDDLE).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn mouse_move(mouse: &mut VirtualMouse, dx: f32, dy: f32) -> bool {
    let dx = (dx * 5.0) as i32;
    let dy = (dy * 5.0) as i32;
    if (dx == 0) && (dy == 0) {
        return false;
    }

    mouse.move_mouse(dx, dy).unwrap();
    true
}

#[cfg(not(target_os = "windows"))]
pub fn handle_mouse_move(message: &str, host: &str, mouse: &mut VirtualMouse) -> bool {
    let offset_str = message
        .replace(&format!("mouse move {} ", host), "")
        .replace("mouse move all ", "");
    let offsets: Vec<&str> = offset_str.split_whitespace().collect();

    match (offsets.get(0), offsets.get(1)) {
        (Some(dx_str), Some(dy_str)) => {
            if let (Ok(dx), Ok(dy)) = (dx_str.parse::<f32>(), dy_str.parse::<f32>()) {
                return mouse_move(mouse, dx, dy);
            } else {
                eprintln!("Error: Invalid offset values");
            }
        }
        _ => {
            eprintln!("Error: Invalid offset format");
        }
    }
    true
}
