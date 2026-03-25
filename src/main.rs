mod analyzer;
mod cli;
mod parser;
mod reader;
mod types;

use clap::Parser;
use cli::Cli;
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
                        stats.on_parse_errors();
                        eprintln!("parse error at line {}: {}", line_no, err);
                    }
                }
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    println!("=== Summary ===");
    println!("Total lines: {}", stats.total_lines);
    println!("Parsed lines: {}", stats.parsed_lines);
    println!("Parse errors: {}", stats.parsed_errors);

    println!("\nStatus counts:");
    let mut status_items: Vec<(u16, usize)> =
        stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
    status_items.sort_by_key(|(code, _)| *code);

    for (code, count) in status_items {
        println!("  {} -> {}", code, count);
    }
    println!("\nTop paths:");
    for (path, count) in stats.top_paths_sorted(10) {
        println!("  {} -> {}", path, count);
    }
    Ok(())
}
