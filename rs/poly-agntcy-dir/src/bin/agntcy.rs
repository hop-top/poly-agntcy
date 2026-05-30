use std::sync::Arc;

use clap::{Parser, Subcommand};
use poly_agntcy_dir::proto::AgentDescriptor;
use poly_agntcy_dir::{
    Client, DescribeParams, DiscoverParams, InsecureCredentials, Options, PublishParams,
    RegisterParams, VerifyParams,
};

#[derive(Parser)]
#[command(name = "agntcy", version, about = "AGNTCY Directory Service CLI")]
struct Cli {
    #[arg(long, env = "AGNTCY_ENDPOINT")]
    endpoint: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Register {
        #[arg(long)]
        name: String,
        #[arg(long)]
        capability: String,
    },
    Discover {
        #[arg(long)]
        capability: String,
    },
    Describe {
        #[arg(long)]
        name: String,
    },
    Publish {
        #[arg(long)]
        name: String,
        #[arg(long)]
        capability: String,
    },
    Verify {
        #[arg(long)]
        envelope: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::new(Options {
        endpoint: cli.endpoint,
        credentials: Arc::new(InsecureCredentials),
    })?;

    match cli.cmd {
        Cmd::Register { name, capability } => {
            let r = client
                .register(RegisterParams {
                    descriptor: AgentDescriptor { name, capability },
                })
                .await?;
            println!("{}", r.name);
        }
        Cmd::Discover { capability } => {
            let r = client.discover(DiscoverParams { capability }).await?;
            for a in r.agents {
                println!("{}", a.name);
            }
        }
        Cmd::Describe { name } => {
            let r = client.describe(DescribeParams { name }).await?;
            if let Some(d) = r.descriptor {
                println!("{}\t{}", d.name, d.capability);
            }
        }
        Cmd::Publish { name, capability } => {
            let r = client
                .publish(PublishParams {
                    descriptor: AgentDescriptor { name, capability },
                })
                .await?;
            println!("{}", r.digest);
        }
        Cmd::Verify { envelope } => {
            let r = client
                .verify(VerifyParams {
                    envelope: envelope.into_bytes(),
                })
                .await?;
            println!("{}", r.valid);
        }
    }
    Ok(())
}
