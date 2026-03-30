use crate::types::LogFormat;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "oculus", version, about = "Analyze log files")]
pub struct Cli {
    pub file: PathBuf,

    #[arg(long, value_enum, default_value_t = LogFormat::Auto)]
    pub format: LogFormat,

    #[arg(long)]
    pub status: Option<u16>,

    #[arg(long)]
    pub contains: Option<String>,

    #[arg(long)]
    pub regex: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
