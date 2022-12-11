mod behavior;

use anyhow::Result;
use behavior::BehaviorEvent;
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity, mdns, mplex, noise,
    swarm::{Swarm, SwarmEvent},
    tcp, Multiaddr, PeerId, Transport,
};

pub struct SwarmBuilder {
    inner: Swarm<behavior::Behavior>,
}

impl SwarmBuilder {
    pub fn new(id: &identity::Keypair) -> Result<Self> {
        // Create a random PeerId
        let peer_id = PeerId::from(id.public());
        log::info!("Local peer id: {peer_id:?}");

        // Create a tokio-based TCP transport use noise for authenticated
        // encryption and Mplex for multiplexing of substreams on a TCP stream.
        let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                noise::NoiseAuthenticated::xx(&id)
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        let mdns_behaviour = mdns::Behaviour::new(Default::default())?;
        let behavior = behavior::Behavior {
            floodsub: Floodsub::new(peer_id),
            mdns: mdns_behaviour,
        };
        let swarm = Swarm::with_tokio_executor(transport, behavior, peer_id);
        Ok(SwarmBuilder {
            inner: swarm,
        })
    }

    pub fn dial(mut self, addr: &str) -> Result<Self> {
        let parsed_addr: Multiaddr = addr.parse()?;
        self.inner.dial(parsed_addr)?;
        log::info!("Dialed {addr:?}");
        Ok(self)
    }

    pub fn listen(mut self) -> Result<ConnectedSwarm> {
        self.inner.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(ConnectedSwarm {
            inner: self.inner,
        })
    }
}

pub struct ConnectedSwarm {
    inner: Swarm<behavior::Behavior>,
}

pub struct ReceivedMessage {
    pub topics: Vec<Topic>,
    pub data: Vec<u8>,
}

impl ConnectedSwarm {
    pub fn subscribe(&mut self, topic: &str) {
        self.inner.behaviour_mut().floodsub.subscribe(Topic::new(topic));
    }

    pub fn publish<T: Serialize>(&mut self, topic: &str, value: &T) {
        let data = serde_json::serialize(value);
        self.inner.behaviour_mut().floodsub.publish(topic, data)
    }

    pub async fn next(&mut self) -> Result<ReceivedMessage> {
        loop {
            let ev = self.inner.next().await.ok_or_else(|| anyhow::anyhow!("Finished"))?;
            match ev {
                SwarmEvent::NewListenAddr { address, .. } => {
                    log::info!("Listening on {address:?}");
                }
                SwarmEvent::Behaviour(BehaviorEvent::Floodsub(FloodsubEvent::Message(ev))) => {
                    return Ok(ReceivedMessage { topics: ev.topics, data: ev.data });
                }
                SwarmEvent::Behaviour(BehaviorEvent::Mdns(event)) => {
                    match event {
                        mdns::Event::Discovered(list) => {
                            for (peer, _) in list {
                                self.inner.behaviour_mut().floodsub.add_node_to_partial_view(peer);
                            }
                        }
                        mdns::Event::Expired(list) => {
                            for (peer, _) in list {
                                if !self.inner.behaviour().mdns.has_node(&peer) {
                                    self.inner.behaviour_mut().floodsub.remove_node_from_partial_view(&peer);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}