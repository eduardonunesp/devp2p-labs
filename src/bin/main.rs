use anyhow::Result;
use tokio_handshake::Discovery;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let disc = Discovery::connect().await?;
    let mut receiver = disc.lookup().await?;

    while let Some(node_record) = receiver.recv().await {
        tracing::info!("{:?}", node_record);
    }

    Ok(())
}
