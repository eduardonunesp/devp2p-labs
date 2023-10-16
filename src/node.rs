use crate::errors::EthereumHanshakeError;
use anyhow::format_err;
use std::{convert::AsRef, error::Error, net::SocketAddr};
use tokio::net::lookup_host;
use url::{ParseError, Url};

const ENODE_SCHEME: &'static str = "enode";

#[derive(Debug)]
pub struct Node {
    id: String,
    host: String,
    port: u16,
    socket_addr: Option<SocketAddr>,
}

impl Node {
    pub fn new<T: AsRef<str>>(enode_id: T) -> Result<Self, EthereumHanshakeError> {
        let result = Url::parse(enode_id.as_ref())?;

        if result.scheme() != ENODE_SCHEME {
            return Err(EthereumHanshakeError::FailedToParseNodeScheme);
        }

        Ok(Self {
            id: result.username().into(),
            host: result
                .host()
                .ok_or(EthereumHanshakeError::NoHostFound)?
                .to_string(),
            port: result.port().unwrap_or_else(|| 30303),
            socket_addr: None,
        })
    }

    pub async fn connect(&mut self) -> Result<(), EthereumHanshakeError> {
        let mut socket_addr = lookup_host(format!("{}:{}", self.host, self.port)).await?;
        self.socket_addr = Some(socket_addr.next().ok_or(
            EthereumHanshakeError::NoSocketAddrFound(self.host.clone(), self.port),
        )?);
        Ok(())
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.host, self.port).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_node() {
        let node = Node::new(
            "enode://d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666@localhost:30303"
                .to_string(),
        )
        .unwrap();
        assert_eq!(node.id, "d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666");
        assert_eq!(node.host, "localhost");
        assert_eq!(node.port, 30303);
    }

    #[test]
    fn test_invalid_node() {
        let node = Node::new("enode://:30303".to_string());
        assert!(node.is_err());

        let node = Node::new("https://localhost:30303".to_string());
        assert!(node.is_err());

        let node = Node::new("oiksoks".to_string());
        assert!(node.is_err());
    }
}
