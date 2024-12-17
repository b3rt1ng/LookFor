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

fn main() {
    let args = Args::parse();

    let mut multi_writer = MultiWriter::new();
    multi_writer.add(io::stdout());

    if let Some(output_path) = &args.output {
        let file = std::fs::File::create(output_path).expect("Unable to create output file");
        multi_writer.add(file);
    }

    let keywords: Vec<String> = args.find.split(',').map(String::from).collect();

    // Compteur pour suivre les occurrences
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
}
