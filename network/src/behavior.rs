use libp2p::{floodsub::{Floodsub, FloodsubEvent}, mdns, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BehaviorEvent")]
pub struct Behavior {
    pub floodsub: Floodsub,
    pub mdns: mdns::tokio::Behaviour,
}

pub enum BehaviorEvent {
    Floodsub(FloodsubEvent),
    Mdns(mdns::Event),
}

impl From<FloodsubEvent> for BehaviorEvent {
    fn from(event: FloodsubEvent) -> Self {
        BehaviorEvent::Floodsub(event)
    }
}

impl From<mdns::Event> for BehaviorEvent {
    fn from(event: mdns::Event) -> Self {
        BehaviorEvent::Mdns(event)
    }
}