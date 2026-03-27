mod analyzer;
mod cli;
mod output;
mod parser;
mod reader;
mod types;

use clap::Parser;
use cli::Cli;
use output::terminal::print_summary;
use parser::LogParser;
use parser::apache::ApacheParser;
use reader::LogReader;
use types::Stats;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut reader = LogReader::new(&args.file)?;
    let parser = ApacheParser::new();
    let mut stats = Stats::default();

    for line_result in reader.lines() {
        match line_result {
            Ok((line_no, line)) => {
                stats.on_line_read();

                match parser.parse(&line) {
                    Ok(entry) => stats.on_parsed_entry(&entry),
                    Err(err) => {
                        stats.on_parse_error();
                        eprintln!("parse error at line {}: {}", line_no, err);
                    }
                }
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }
    print_summary(&stats);

    Ok(())
}
