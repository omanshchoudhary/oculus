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
use parser::detector::detect_format;
use parser::json::JsonParser;
use parser::nginx::NginxParser;
use reader::LogReader;
use std::path::Path;
use types::Stats;

use crate::types::LogFormat;

fn collect_sample_lines(path: &Path, limit: usize) -> anyhow::Result<Vec<String>> {
    let mut reader = LogReader::new(path)?;
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        let (_, line) = line_result?;
        if line.trim().is_empty() {
            continue;
        }

        lines.push(line);
        if lines.len() >= limit {
            break;
        }
    }

    Ok(lines)
}

fn build_parser(format: LogFormat) -> Box<dyn LogParser> {
    match format {
        LogFormat::Apache => Box::new(ApacheParser::new()),
        LogFormat::Nginx => Box::new(NginxParser::new()),
        LogFormat::Json => Box::new(JsonParser::new()),
        LogFormat::Auto => unreachable!("auto format must be resolved before parser creation"),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let selected_format = match args.format {
        LogFormat::Auto => {
            let sample_lines = collect_sample_lines(&args.file, 50)?;
            detect_format(&sample_lines)
        }
        format => format,
    };
    if args.verbose {
        eprintln!("using format: {:?}", selected_format);
    }

    let parser = build_parser(selected_format);
    let mut reader = LogReader::new(&args.file)?;
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
