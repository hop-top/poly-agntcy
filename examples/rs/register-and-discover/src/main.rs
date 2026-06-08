use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use hop_top_agntcy_dir::{
    Client, DiscoverParams, InsecureCredentials, Options, PublishParams, RegisterParams,
};
use hop_top_agntcy_dir::proto::AgentDescriptor;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "http://localhost:8888")]
    endpoint: String,
    #[arg(long, default_value = "inventory")]
    capability: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new(Options {
        endpoint: args.endpoint,
        credentials: Arc::new(InsecureCredentials),
    })?;

    let descriptor = AgentDescriptor {
        name: "inventory-agent".into(),
        capabilities: vec![args.capability.clone()],
        ..Default::default()
    };

    let reg = client
        .register(RegisterParams { descriptor: descriptor.clone() })
        .await?;
    println!("registered name={}", reg.name);

    let dis = client
        .discover(DiscoverParams { capability: args.capability })
        .await?;
    for a in dis.agents {
        println!("discovered {}", a.name);
    }

    client
        .publish(PublishParams { descriptor })
        .await?;
    println!("published signed payload");
    Ok(())
}
