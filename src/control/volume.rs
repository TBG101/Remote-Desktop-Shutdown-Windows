use enigo::Keyboard;

use crate::{
    control::input::get_enigo,
    network::payload::{VolumeAction, VolumeControl},
};

pub fn control_volume(payload: VolumeControl) {
    let enigo = get_enigo();

    let action = match payload.action {
        VolumeAction::Up => enigo::Key::VolumeUp,
        VolumeAction::Down => enigo::Key::VolumeDown,
        VolumeAction::Mute => enigo::Key::VolumeMute,
    };

    let _ = enigo.lock().unwrap().key(action, enigo::Direction::Click);
}
