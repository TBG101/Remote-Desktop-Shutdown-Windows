use std::sync::OnceLock;

use mdns_sd::{ServiceDaemon, ServiceInfo};

pub const SERVICE_TYPE: &str = "_rdesk._udp.local.";
pub const COMMAND_PORT: u16 = 8888;

static DAEMON: OnceLock<ServiceDaemon> = OnceLock::new();

pub fn advertise(hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;

    let info = ServiceInfo::new(
        SERVICE_TYPE,
        hostname,
        &format!("{hostname}.local."),
        "",
        COMMAND_PORT,
        None,
    )?
    .enable_addr_auto();

    mdns.register(info)?;

    // Keep alive
    let _ = DAEMON.set(mdns);
    Ok(())
}

pub fn shutdown() {
    if let Some(daemon) = DAEMON.get() {
        let _ = daemon.shutdown();
    }
}
