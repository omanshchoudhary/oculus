use crate::types::{LogFormat, OutputFormat};
use clap::Parser;
use std::path::PathBuf;

const EXAMPLES: &str = "\
EXAMPLES:
    # Analyze a log file (format auto-detected)
    oculus access.log

    # Force a specific format
    oculus app.log --format json

    # Filter by HTTP status code
    oculus access.log --status 500

    # Filter by substring or regex
    oculus access.log --contains /api/users
    oculus access.log --regex \"GET /api/v[0-9]+\"

    # Export a JSON summary to a file
    oculus access.log --output json --output-file report.json

    # Strict mode: exit non-zero if any line fails to parse
    oculus access.log --fail-on-parse-errors";

// tells you what inputs the program accepts
#[derive(Debug, Parser)]
#[command(
    name = "oculus",
    version,
    about = "Analyze log files",
    after_help = EXAMPLES
)]
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

    #[arg(long, default_value_t = false)]
    pub no_color: bool,
}
