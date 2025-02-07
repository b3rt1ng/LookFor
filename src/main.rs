mod args;
mod search;
mod writer;
mod utils;

use args::Args;
use search::{search_in_directory, search_in_file};
use writer::MultiWriter;
use std::io;
use std::io::Write;
use clap::Parser;
use std::collections::HashMap;
use colored::*;
use std::time::Instant;
use regex::Regex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("{}", "\nReceived Ctrl+C, finishing current operation...".bright_red().bold());
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let mut analyzed_files = 0;

    let find_value = args.find.or(args.positional_find).unwrap_or_else(|| {
        eprintln!("Error: No search keyword provided.");
        std::process::exit(1);
    });

    let keywords: Vec<String> = find_value.split(',').map(String::from).collect();

    let regex_list: Vec<Regex> = args
        .regex
        .as_ref() 
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|pattern| Regex::new(pattern).ok())
        .collect();

    let mut multi_writer = MultiWriter::new();
    multi_writer.add(io::stdout());

    if let Some(output_path) = &args.output {
        let file = std::fs::File::create(output_path).expect("Unable to create output file");
        multi_writer.add(file);
    }

    let mut keyword_counts: HashMap<String, usize> = keywords
        .iter()
        .map(|keyword| (keyword.clone(), 0))
        .collect();

    let mut regex_counts: Vec<usize> = vec![0; regex_list.len()];

    if args.path.is_dir() {
        search_in_directory(
            &keywords,
            &args.path,
            args.show,
            args.maxsize,
            &mut multi_writer,
            &mut keyword_counts,
            &mut regex_counts,
            &regex_list,
            &args.omit,
            running.clone(),
            &mut analyzed_files,
        );
    } else if args.path.is_file() {
        search_in_file(
            &keywords,
            &args.path,
            args.show,
            args.maxsize,
            &mut multi_writer,
            &mut keyword_counts,
            &mut regex_counts,
            &regex_list,
            running.clone(),
            &mut analyzed_files,
        )
        .unwrap_or_else(|e| {
            if args.show {
                eprintln!("Error with the file {}: {}", args.path.display(), e);
            }
        });
    }

    writeln!(&mut multi_writer, "\nSummary:").unwrap();

    writeln!(
        &mut multi_writer,
        "Files analyzed: {}",
        analyzed_files.to_string().green().bold()
    ).unwrap();
    
    for (keyword, count) in &keyword_counts {
        writeln!(
            &mut multi_writer,
            "Keyword '{}' found {} times",
            keyword.red().bold(),
            count.to_string().green().bold(),
        )
        .unwrap();
    }

    for (i, count) in regex_counts.iter().enumerate() {
        writeln!(
            &mut multi_writer,
            "Regex pattern '{}' matched {} times",
            args.regex.as_ref().unwrap()[i].red().bold(),
            count.to_string().green().bold(),
        )
        .unwrap();
    }

    let elapsed = start_time.elapsed();
    let hours = elapsed.as_secs() / 3600;
    let minutes = (elapsed.as_secs() % 3600) / 60;
    let seconds = elapsed.as_secs() % 60;
    let milliseconds = elapsed.subsec_millis();

    let elapsed_str = if hours > 0 {
        format!("{:02}h:{:02}m:{:02}s.{:03}ms", hours, minutes, seconds, milliseconds)
    } else if minutes > 0 {
        format!("{:02}m:{:02}s.{:03}ms", minutes, seconds, milliseconds)
    } else if seconds > 0 {
        format!("{:02}s.{:03}ms", seconds, milliseconds)
    } else {
        format!("{}ms", milliseconds)
    };

    writeln!(&mut multi_writer, "\nElapsed time: {}", elapsed_str).unwrap();
}
