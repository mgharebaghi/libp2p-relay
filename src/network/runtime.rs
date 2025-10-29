use anyhow::Result;
use libp2p::{PeerId, Swarm, futures::StreamExt, identify, kad, swarm::SwarmEvent};
use tracing::{info, warn};

use crate::network::behaviours::{RelayServerBehaviour, RelayServerBehaviourEvent};

pub async fn start_network(
    swarm: &mut Swarm<RelayServerBehaviour>,
    _local_peer_id: PeerId,
) -> Result<()> {
    // Placeholder for network runtime logic
    loop {
        match swarm.select_next_some().await {
            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                if !address.to_string().contains("127.0.0.1") {
                    info!("Listening on {:?}", address);
                    swarm.add_external_address(address);
                }
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("Connection established with {:?}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                warn!("Connection closed with: {}", peer_id);
                warn!("Cause: {:?}", cause);
                swarm.behaviour_mut().kad.remove_peer(&peer_id);
            }
            libp2p::swarm::SwarmEvent::Behaviour(event) => match event {
                RelayServerBehaviourEvent::Relay(ev) => {
                    info!("Relay Events: {:?}", ev);
                }
                RelayServerBehaviourEvent::Identify(ev) => match ev {
                    identify::Event::Received { info, peer_id, .. } => {
                        if info.listen_addrs.len() > 0 {
                            swarm
                                .behaviour_mut()
                                .kad
                                .add_address(&peer_id, info.listen_addrs[0].clone());
                        }
                    }
                    _ => {}
                },
                RelayServerBehaviourEvent::Ping(_ev) => {
                    // info!("ping: {:?}", ev);
                }
                RelayServerBehaviourEvent::Kad(ev) => match ev {
                    kad::Event::RoutingUpdated {
                        peer,
                        is_new_peer,
                        addresses,
                        bucket_range,
                        old_peer,
                    } => {
                        info!(
                            "Kademlia routing updated: peer={:?}, is_new_peer={}, addresses={:?}, bucket_range={:?}, old_peer={:?}",
                            peer, is_new_peer, addresses, bucket_range, old_peer
                        );
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
}
