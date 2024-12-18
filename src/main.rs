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

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();

    let mut multi_writer = MultiWriter::new();
    multi_writer.add(io::stdout());

    if let Some(output_path) = &args.output {
        let file = std::fs::File::create(output_path).expect("Unable to create output file");
        multi_writer.add(file);
    }

    let keywords: Vec<String> = args.find.split(',').map(String::from).collect();
    
    let omit_extensions: Vec<String> = args
        .omit
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect();

    let regex = args
        .regex
        .as_ref()
        .and_then(|pattern| Regex::new(pattern).ok());

    let mut keyword_counts: HashMap<String, usize> = keywords
        .iter()
        .map(|keyword| (keyword.clone(), 0))
        .collect();

    if args.path.is_dir() {
        writeln!(
            &mut multi_writer,
            "Searching in directory: {}",
            args.path.display()
        )
        .unwrap();
        search_in_directory(
            &keywords,
            &args.path,
            args.noshow,
            args.maxsize,
            &mut multi_writer,
            &mut keyword_counts,
            &omit_extensions,
            regex.as_ref(), // Pass regex
        );
    } else if args.path.is_file() {
        writeln!(
            &mut multi_writer,
            "Searching in file: {}",
            args.path.display()
        )
        .unwrap();
        if let Err(e) = search_in_file(
            &keywords,
            &args.path,
            args.noshow,
            args.maxsize,
            &mut multi_writer,
            &mut keyword_counts,
            regex.as_ref(), // Pass regex
        ) {
            if !args.noshow {
                eprintln!("Error with the file {}: {}", args.path.display(), e);
            }
        }
    }


    // Affichage des résultats du compteur
    writeln!(&mut multi_writer, "\nSummary:").unwrap();
    for (keyword, count) in &keyword_counts {
        let times_str = if *count > 1 { "times" } else { "time" };
        writeln!(
            &mut multi_writer,
            "found {}: {} {}",
            keyword.red().bold(),
            count.to_string().green().bold(),
            times_str
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
