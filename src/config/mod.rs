use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) server: Server,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Server {
    pub(crate) address: IpAddr,
    pub(crate) port: u16,
}
