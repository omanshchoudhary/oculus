use crate::types::{LogFormat, OutputFormat};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "oculus", version, about = "Analyze log files")]
pub struct Cli {
    pub file: PathBuf,

    #[arg(long, value_enum, default_value_t = LogFormat::Auto)]
    pub format: LogFormat,

    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    pub output: OutputFormat,

    #[arg(long)]
    pub output_file: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    pub force: bool,

    #[arg(long)]
    pub status: Option<u16>,

    #[arg(long)]
    pub contains: Option<String>,

    #[arg(long)]
    pub regex: Option<String>,

    #[arg(long)]
    pub from: Option<String>,

    #[arg(long)]
    pub to: Option<String>,

    #[arg(long, default_value_t = false)]
    pub fail_on_parse_errors: bool,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(long)]
    pub ip: Option<String>,

    #[arg(long)]
    pub cidr: Option<String>,
}
