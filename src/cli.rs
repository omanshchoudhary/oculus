use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "oculus", version, about = "Analyze log files")]
pub struct Cli {
    pub file: PathBuf,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
