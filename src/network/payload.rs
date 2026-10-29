use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::io;

// ==========================================
// MOUSE MOVEMENT DATA STRUCTURE
// ==========================================

#[derive(Debug, Deserialize)]
pub struct MouseMove {
    pub dx: i32,
    pub dy: i32,
}

// ==========================================
// MOUSE BUTTON DEFINITIONS & CONVERSIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8")]
pub enum MouseButton {
    Left = 0x00,
    Right = 0x01,
    Middle = 0x02,
}

impl MouseButton {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for MouseButton {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::Left),
            0x01 => Ok(Self::Right),
            0x02 => Ok(Self::Middle),
            other => Err(other),
        }
    }
}

// ==========================================
// CLICK ACTION DEFINITIONS & CONVERSIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8")]
pub enum ClickAction {
    Click = 0x00,
    Press = 0x01,
    Release = 0x02,
}

impl TryFrom<u8> for ClickAction {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::Click),
            0x01 => Ok(Self::Press),
            0x02 => Ok(Self::Release),
            other => Err(other),
        }
    }
}

// ==========================================
// MOUSE CLICK & SCROLL STRUCTURES
// ==========================================

#[derive(Debug, Deserialize)]
pub struct MouseClick {
    pub button: MouseButton,
    pub action: ClickAction,
}

#[derive(Debug, Deserialize)]
pub struct MouseScroll {
    pub amount: i32,
    pub axis: i32, // 1 means vertical, 0 means horizontal
}

// ==========================================
// VOLUME ACTION DEFINITIONS & CONVERSIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8")]
pub enum VolumeAction {
    Up,
    Down,
    Mute,
}

impl TryFrom<u8> for VolumeAction {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::Mute),
            0x01 => Ok(Self::Down),
            0x02 => Ok(Self::Up),
            other => Err(other),
        }
    }
}

// ==========================================
// VOLUME CONTROL DATA STRUCTURE
// ==========================================

#[derive(Debug, Deserialize)]
pub struct VolumeControl {
    pub action: VolumeAction,
}

// ==========================================
// MEDIA ACTION DEFINITIONS & CONVERSIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8")]
pub enum MediaAction {
    STOP,
    PLAY,
    NEXT,
    PREVIOUS,
}

impl TryFrom<u8> for MediaAction {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::STOP),
            0x01 => Ok(Self::PLAY),
            0x02 => Ok(Self::NEXT),
            0x03 => Ok(Self::PREVIOUS),
            other => Err(other),
        }
    }
}

// ==========================================
// MEDIA CONTROL DATA STRUCTURE
// ==========================================

#[derive(Debug, Deserialize)]
pub struct MediaControl {
    pub action: MediaAction,
}

// ==========================================
// PAYLOAD DECODING UTILITY
// ==========================================

/// Decodes a serialized binary payload into any type implementing `DeserializeOwned`.
pub fn decode<T: DeserializeOwned>(payload: &[u8]) -> io::Result<T> {
    rmp_serde::from_slice(payload).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
