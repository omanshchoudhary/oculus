use anyhow::anyhow;
use clap::Parser;
use oculus::cli::Cli;
use oculus::filter::{FilterConfig, FilterEngine};
use oculus::output::Report;
use oculus::output::csv::render_csv;
use oculus::output::json::render_json;
use oculus::output::terminal::render_table;
use oculus::parser::LogParser;
use oculus::parser::apache::ApacheParser;
use oculus::parser::detector::detect_format;
use oculus::parser::json::JsonParser;
use oculus::parser::nginx::NginxParser;
use oculus::reader::LogReader;
use oculus::types::Stats;
use oculus::types::{LogFormat, OutputFormat};
use std::fs;
use std::io::IsTerminal;
use std::path::Path;

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

// Building parser based upon the provided format
fn build_parser(format: LogFormat, parse_timestamp: bool) -> Box<dyn LogParser> {
    match format {
        LogFormat::Apache => Box::new(ApacheParser::new().with_timestamps(parse_timestamp)),
        LogFormat::Nginx => Box::new(NginxParser::new()),
        LogFormat::Json => Box::new(JsonParser::new()),
        LogFormat::Auto => unreachable!("auto format must be resolved before parser creation"),
    }
}

// Execution starts here
fn main() -> anyhow::Result<()> {
    // Inputs from the cli
    let args = Cli::parse();
    // finalize the format for the lines
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

    // The timestamp is only read by the time filters, so skip parsing it
    // otherwise (it is ~25% of per-line cost — see docs/benchmarks.md).
    let need_timestamp = args.from.is_some() || args.to.is_some();
    let parser = build_parser(selected_format, need_timestamp);

    let filters = FilterEngine::new(FilterConfig {
        status: args.status,
        contains: args.contains,
        regex: args.regex,
        from: args.from,
        to: args.to,
        ip: args.ip,
        cidr: args.cidr,
    })?;

    let mut reader = LogReader::new(&args.file)?;
    let mut stats = Stats::default();

    for line_result in reader.lines() {
        match line_result {
            Ok((line_no, line)) => {
                stats.on_line_read();

                match parser.parse(&line) {
                    Ok(entry) => {
                        if filters.accept(&entry) {
                            stats.on_parsed_entry(&entry);
                        }
                    }
                    Err(err) => {
                        stats.on_parse_error(&err);
                        if args.verbose {
                            eprintln!("parse error at line {}: {}", line_no, err);
                        }
                    }
                }
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }
    let report = Report::from_stats(&stats);
    let use_color = !args.no_color && args.output_file.is_none() && std::io::stdout().is_terminal();

    let rendered_output = match args.output {
        OutputFormat::Table => render_table(&report, use_color),
        OutputFormat::Json => render_json(&report)?,
        OutputFormat::Csv => render_csv(&report),
    };

    if let Some(path) = args.output_file {
        if path.exists() && !args.force {
            return Err(anyhow!(
                "output file '{}' already exists; use --force to overwrite",
                path.display()
            ));
        }
        fs::write(&path, rendered_output)?;
    } else {
        print!("{}", rendered_output);
    }

    if args.fail_on_parse_errors && stats.parse_errors > 0 {
        return Err(anyhow!(
            "encountered {} parse error(s) with strict mode enabled",
            stats.parse_errors
        ));
    }
    Ok(())
}
