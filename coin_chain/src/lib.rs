mod block;
mod chain;

use anyhow::Result;

pub use block::Data;

pub struct App {
    swarm: network::ConnectedSwarm,
    chain: chain::Chain,
}

const TOPIC_CHAIN: &str = "chain";
const TOPIC_BLOCK: &str = "block";

impl App {
    pub fn new(mut swarm: network::ConnectedSwarm) -> Self {
        swarm.subscribe(TOPIC_CHAIN);
        swarm.subscribe(TOPIC_BLOCK);
        let chain = chain::Chain::new();
        swarm.publish(TOPIC_CHAIN, &chain);
        Self {
            swarm: swarm,
            chain: chain,
        }
    }

    pub async fn mine(&self) -> Result<()> {
        self.chain = self.chain.mine()?;
        self.swarm.publish(TOPIC_BLOCK, &self.last());
        Ok(())
    }

    pub async fn step(&self) -> Result<()> {
        let msg = self.inner.next().await?;
        if msg.topics.contains(TOPIC_CHAIN) {
            let other_chain = serde_json::from_bytes(msg.data)?;
            self.chain = self.chain.choose_chain(other_chain);
        } else {
            let block = serde_json::from_bytes(msg.data)?;
            self.chain = self.chain.append(block);
        }
        Ok(())
    }
}