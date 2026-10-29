#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Shutdown = 0x01,
    ShutdownAll = 0x02,
    MouseMove = 0x03,
    MouseClick = 0x04,
    MouseScroll = 0x05,
    Volume = 0x06,
    Media = 0x07,
}

impl TryFrom<u8> for Command {
    type Error = u8;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        match id {
            0x01 => Ok(Command::Shutdown),
            0x02 => Ok(Command::ShutdownAll),
            0x03 => Ok(Command::MouseMove),
            0x04 => Ok(Command::MouseClick),
            0x05 => Ok(Command::MouseScroll),
            0x06 => Ok(Command::Volume),
            0x07 => Ok(Command::Media),
            other => Err(other),
        }
    }
}
