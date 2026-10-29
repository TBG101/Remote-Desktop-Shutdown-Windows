use enigo::{Axis, Button, Coordinate, Direction, Mouse};

use crate::{
    control::input::get_enigo,
    network::payload::{ClickAction, MouseButton, MouseClick, MouseScroll},
};

pub fn move_mouse(x: i32, y: i32) {
    let enigo = get_enigo();

    let _ = enigo.lock().unwrap().move_mouse(x, y, Coordinate::Rel);
}

pub fn button_click(mouse: MouseClick) {
    let button = match mouse.button {
        MouseButton::Left => Button::Left,
        MouseButton::Right => Button::Right,
        MouseButton::Middle => Button::Middle,
    };

    let direction = match mouse.action {
        ClickAction::Click => Direction::Click,
        ClickAction::Press => Direction::Press,
        ClickAction::Release => Direction::Release,
    };

    let mut enigo = get_enigo().lock().unwrap();
    let _ = enigo.button(button, direction);
}

pub fn scroll(mouse_scroll: MouseScroll) {
    let mut enigo = get_enigo().lock().unwrap();
    let axis = match mouse_scroll.axis {
        1 => Axis::Vertical,
        0 => Axis::Horizontal,
        _ => Axis::Vertical,
    };

    let _ = enigo.scroll(mouse_scroll.amount, axis);
}
