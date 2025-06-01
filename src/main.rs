use anyhow::Result;
use clap::{Parser};
use processor_inbound::config::commands::{Cli, Commands};
use tracing::{info};

#[tokio::main]
async fn main()  -> Result<()> {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Process {
            input,
            format,
            channel,
            data_type,
            tenant
        } => {
            info!("Processing file {:?}", input);
        }
    }


    Ok(())
}
