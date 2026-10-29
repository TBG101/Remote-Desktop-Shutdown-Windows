use enigo::Keyboard;

use crate::{
    control::input::get_enigo,
    network::payload::{MediaAction, MediaControl},
};

pub fn control_media(playload: MediaControl) {
    let enigo = get_enigo();

    let action = match playload.action {
        MediaAction::NEXT => enigo::Key::MediaNextTrack,
        MediaAction::PREVIOUS => enigo::Key::MediaPrevTrack,
        MediaAction::PLAY => enigo::Key::MediaPlayPause,
        MediaAction::STOP => enigo::Key::MediaStop,
    };

    let _ = enigo.lock().unwrap().key(action, enigo::Direction::Click);
}
