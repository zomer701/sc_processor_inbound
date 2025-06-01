use std::path::{PathBuf};
use clap::{Parser, Subcommand};
use crate::core::types::{FileFormat, Channel, DataType};

#[derive(Parser)]
#[command(name = "processor")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Process {

        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long, value_enum)]
        format: FileFormat,

        #[arg(short, long, value_enum)]
        channel: Channel,

        #[arg(short, long, value_enum)]
        data_type: DataType,

        #[arg(short, long)]
        tenant: String,
    }
}