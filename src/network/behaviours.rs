use libp2p::{identify, kad, ping, relay, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
pub struct RelayServerBehaviour {
    pub relay: relay::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub kad: kad::Behaviour<kad::store::MemoryStore>,
}
