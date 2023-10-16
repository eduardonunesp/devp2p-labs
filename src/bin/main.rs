#![allow(
    dead_code,
    unused,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    unused_parens,
    unused_must_use
)]

use anyhow::Result;
use futures::future::join_all;
use std::net::SocketAddr;
use tokio::net::lookup_host;
use tokio_handshake::node::Node;

const ETHEREUM_NODE: &'static str = "enode://d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666@18.138.108.67:30303";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut node = Node::new(ETHEREUM_NODE)?;
    tracing::info!("{:?}", node);

    // let result = node.connect().await?;
    // tracing::info!("{:?}", result);

    Ok(())
}
