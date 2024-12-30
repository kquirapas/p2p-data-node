use libp2p::{
    swarm::{behaviour, NetworkBehaviour},
    Multiaddr, PeerId,
};

use libp2p::gossipsub::{Behaviour as GossipsubBehavior, Event as GossipsubEvent};
use libp2p::identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent};
use libp2p::kad::{
    store::MemoryStore, Behaviour as KadBehaviour, Event as KadEvent, RoutingUpdate,
};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "Event")]
pub(crate) struct Behaviour {
    // gossipsub: GossipsubBehavior,
    identify: IdentifyBehaviour,
    kad: KadBehaviour<MemoryStore>,
}

impl Behaviour {
    pub fn new(
        // gossipsub: GossipsubBehavior,
        identify: IdentifyBehaviour,
        kad: KadBehaviour<MemoryStore>,
    ) -> Self {
        Self {
            // gossipsub,
            identify,
            kad,
        }
    }

    pub fn add_address(&mut self, peer_id: &PeerId, addr: Multiaddr) -> RoutingUpdate {
        self.kad.add_address(peer_id, addr)
    }
}

#[derive(Debug)]
pub(crate) enum Event {
    // Gossipsub(GossipsubEvent),
    Identify(IdentifyEvent),
    Kad(KadEvent),
}

// impl From<GossipsubEvent> for Event {
//     fn from(value: GossipsubEvent) -> Self {
//         Self::Gossipsub(value)
//     }
// }

impl From<IdentifyEvent> for Event {
    fn from(value: IdentifyEvent) -> Self {
        Self::Identify(value)
    }
}

impl From<KadEvent> for Event {
    fn from(value: KadEvent) -> Self {
        Self::Kad(value)
    }
}
