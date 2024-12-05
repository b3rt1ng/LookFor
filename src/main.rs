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

fn main() {
    let args = Args::parse();

    let mut multi_writer = MultiWriter::new();
    multi_writer.add(io::stdout());

    if let Some(output_path) = &args.output {
        let file = std::fs::File::create(output_path).expect("Unable to create output file");
        multi_writer.add(file);
    }

    let keywords: Vec<String> = args.find.split(',').map(String::from).collect();

    if args.path.is_dir() {
        writeln!(
            &mut multi_writer,
            "Searching in directory: {}",
            args.path.display()
        )
        .unwrap();
        search_in_directory(&keywords, &args.path, args.noshow, args.maxsize, &mut multi_writer);
    } else if args.path.is_file() {
        writeln!(
            &mut multi_writer,
            "Searching in file: {}",
            args.path.display()
        )
        .unwrap();
        if let Err(e) = search_in_file(&keywords, &args.path, args.noshow, args.maxsize, &mut multi_writer) {
            if !args.noshow {
                eprintln!("Error with the file {}: {}", args.path.display(), e);
            }
        }
    }
}
