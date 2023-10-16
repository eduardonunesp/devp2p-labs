use crate::errors::EthereumHanshakeError;
use crate::peer::Peer;

use std::net::SocketAddr;
use tokio::net::lookup_host;

// Try
#[derive(Debug)]
pub struct Node {
    peer: Peer,
    socket_addr: Option<SocketAddr>,
}

impl Node {
    pub fn new(peer: Peer) -> Self {
        Self {
            peer,
            socket_addr: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), EthereumHanshakeError> {
        let mut socket_addr = lookup_host(format!("{}:{}", self.peer.host, self.peer.port)).await?;
        self.socket_addr = Some(socket_addr.next().ok_or(
            EthereumHanshakeError::NoSocketAddrFound(self.peer.host.clone(), self.peer.port),
        )?);

        Ok(())
    }
}
