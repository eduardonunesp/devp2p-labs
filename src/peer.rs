use crate::{capability::CapabilityMessage, errors::EthereumHanshakeError};

use discv4::NodeId;
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use std::convert::AsRef;
use url::Url;

const ENODE_SCHEME: &'static str = "enode";

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub host: String,
    pub port: u16,
}

impl Peer {
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
        })
    }
}

#[derive(Clone, Debug)]
pub struct HelloMessage {
    pub protocol_version: usize,
    pub client_version: String,
    pub capabilities: Vec<CapabilityMessage>,
    pub port: u16,
    pub id: NodeId,
}

impl Encodable for HelloMessage {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(5);
        s.append(&self.protocol_version);
        s.append(&self.client_version);
        s.append_list(&self.capabilities);
        s.append(&self.port);
        s.append(&self.id);
    }
}

impl Decodable for HelloMessage {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Self {
            protocol_version: rlp.val_at(0)?,
            client_version: rlp.val_at(1)?,
            capabilities: rlp.list_at(2)?,
            port: rlp.val_at(3)?,
            id: rlp.val_at(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_peer() {
        let peer = Peer::new(
            "enode://d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666@localhost:30303"
                .to_string(),
        )
        .unwrap();
        assert_eq!(peer.id, "d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666");
        assert_eq!(peer.host, "localhost");
        assert_eq!(peer.port, 30303);
    }

    #[test]
    fn test_invalid_peer() {
        let peer = Peer::new("enode://:30303".to_string());
        assert!(peer.is_err());

        let peer = Peer::new("https://localhost:30303".to_string());
        assert!(peer.is_err());

        let peer = Peer::new("oiksoks".to_string());
        assert!(peer.is_err());
    }
}
