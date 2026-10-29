use crate::network::payload::{
    self, MediaControl, MouseClick, MouseMove, MouseScroll, VolumeControl,
};
use std::io;
use std::net::{SocketAddr, UdpSocket};

use crate::network::command::Command;
use crate::{commands, control};

pub fn handle_packet(
    _socket: &UdpSocket,
    data: &[u8],
    _sender: &SocketAddr,
    _hostname: &str,
) -> io::Result<()> {
    let Some((&command_id, payload)) = data.split_first() else {
        return Ok(());
    };

    _ = match Command::try_from(command_id) {
        Ok(cmd) => dispatch(cmd, payload),
        Err(unknown) => {
            // TODO: Add Logging
            Ok(())
        }
    };

    Ok(())
}

fn dispatch(cmd: Command, payload: &[u8]) -> io::Result<()> {
    match cmd {
        Command::Shutdown | Command::ShutdownAll => commands::shutdown::shutdown(),

        Command::MouseMove => {
            let p: MouseMove = payload::decode(payload)?;
            control::mouse::move_mouse(p.dx, p.dy);
        }
        Command::MouseClick => {
            let p: MouseClick = payload::decode(payload)?;
            control::mouse::button_click(p);
        }

        Command::MouseScroll => {
            let p: MouseScroll = payload::decode(payload)?;
            control::mouse::scroll(p);
        }
        Command::Volume => {
            let p: VolumeControl = payload::decode(payload)?;
            control::volume::control_volume(p);
        }
        Command::Media => {
            let p: MediaControl = payload::decode(payload)?;
            control::media::control_media(p);
        }
    }
    Ok(())
}
