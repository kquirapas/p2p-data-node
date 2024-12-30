mod behavior;

use anyhow::Result;
use behavior::Behaviour;
use futures::prelude::*;
use libp2p::{
    gossipsub::{Behaviour as GossipsubBehavior, Event as GossipsubEvent},
    identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent},
    kad::{
        store::MemoryStore as KadInMemory, Behaviour as KadBehaviour, Event as KadEvent,
        RoutingUpdate,
    },
    swarm::{behaviour, NetworkBehaviour},
    Multiaddr, PeerId,
};
use libp2p::{identify, identity::Keypair, kad, ping, swarm::SwarmEvent, StreamProtocol};
use std::{thread, time::Duration};

const ID_PROTOCOL_STRING: &str = "/sonic/connection/0.1.0";
const KAD_PROTOCOL_STRING: &str = "/sonic/discovery/0.1.0";

#[tokio::main]
async fn main() -> Result<()> {
    // let ping_config = ping::Config::default()
    //     .with_timeout(Duration::from_secs(5))
    //     .with_interval(Duration::from_secs(1));
    //

    let keypair = Keypair::generate_ed25519();

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let local_peer_id = PeerId::from_public_key(&keypair.public());
            let identify_config =
                identify::Config::new(ID_PROTOCOL_STRING.to_string(), keypair.public());
            let identify = identify::Behaviour::new(identify_config);

            let kad_config = kad::Config::new(StreamProtocol::new(KAD_PROTOCOL_STRING));
            let kad_store = kad::store::MemoryStore::new(local_peer_id);
            let kad = kad::Behaviour::new(local_peer_id, kad_store);

            Behaviour::new(identify, kad)
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))) // Allows us to observe pings indefinitely.
        .build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    let mut peers: Vec<PeerId> = Vec::new();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => {}

            _ => {}
        }
    }

    // Ok(())
}
