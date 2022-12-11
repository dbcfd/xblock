use anyhow::Result;
use clap::{Parser, SubCommand};
use libp2p::{mplex, noise, identity, Keypair, TokioTcpConfig, NoiseConfig};

#[derive(SubCommand)]
enum Chain { 
    A,
    B,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    chain: Chain,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let id_keys = identity::Keypair::generate_ed25519();

    match &cli.chain {
        Chain::A => {
            
        }
        Chain::B => {

        }
    }
}

async fn run_chain_a(id_keys: &identity::Keypair) -> Result<()> {
    let swarm = network::SwarmBuilder::new(&id_keys)?;
    let swarm = network.listen()?;
    let chain = coin_chain::App::new(swarm);
    loop {
        chain.step().await?;
    }
}

async fn run_chain_b() -> Result<()> {
    let swarm = network::SwarmBuilder::new(&id_keys)?;
    let swarm = network.listen()?;
    let chain = contract_chain::App::new(swarm);
    loop {
        chain.step().await?;
    }
}

async fn serve() -> Result<()> {
    actix_web::HttpServer::new(move || {
        chain::app()
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
