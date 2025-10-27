use anyhow::Result;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::SwarmBuilder;
use libp2p::identify;
use libp2p::kad;
use libp2p::noise;
use libp2p::ping;
use libp2p::relay;
use libp2p::swarm::Swarm;
use libp2p::tcp;
use libp2p::yamux;

use crate::network::behaviours::RelayServerBehaviour;

pub async fn swarm_for_relay() -> Result<(Swarm<RelayServerBehaviour>, PeerId)> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());

    let behaviour = RelayServerBehaviour {
        relay: relay::Behaviour::new(local_peer_id, relay::Config::default()),
        identify: identify::Behaviour::new(identify::Config::new(
            "/relay/0.0.1".to_string(),
            local_key.public(),
        )),
        ping: ping::Behaviour::default(),
        kad: kad::Behaviour::new(local_peer_id, kad::store::MemoryStore::new(local_peer_id)),
    };

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, || {
            yamux::Config::default()
        })?
        .with_behaviour(|_| behaviour)?
        .build();

    let multiaddr: Multiaddr = format!("/ip4/0.0.0.0/tcp/0/p2p/{}", local_peer_id).parse()?;
    swarm.listen_on(multiaddr)?;

    let bootstrap_relay: Multiaddr = "/ip4/192.168.1.120/tcp/55402".parse()?;
    swarm.dial(bootstrap_relay)?;

    Ok((swarm, local_peer_id))
}
