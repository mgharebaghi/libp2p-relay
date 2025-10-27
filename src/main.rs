use anyhow::Result;
mod infra;
use infra::init_logging;

use crate::network::runtime::start_network;
mod network;
use network::config::swarm_for_relay;

#[tokio::main]
async fn main() -> Result<()> {
    let _log_guard = init_logging()?;
    let (mut swarm, local_peer_id) = swarm_for_relay().await?;

    start_network(&mut swarm, local_peer_id).await?;

    Ok(())
}
