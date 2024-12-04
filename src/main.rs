use clap::Parser;
use std::fs;
use std::io::{self, BufRead, BufReader};
use walkdir::WalkDir;
use colored::*;
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Parser)]
struct Args {
    keyword: String,

    paths: Vec<PathBuf>,

    #[clap(long)]
    noshow: bool,
}

fn main() {
    let args = Args::parse();

    for path in &args.paths {
        if path.is_dir() {
            println!("Searching in directory: {}", path.display());
            search_in_directory(&args.keyword, path, args.noshow);
        } else if path.is_file() {
            println!("Searching in file: {}", path.display());
            if let Err(e) = search_in_file(&args.keyword, path, args.noshow) {
                if !args.noshow {
                    eprintln!("Error with file {}: {}", path.display(), e);
                }
            }
        }
    }
}

fn search_in_directory(keyword: &str, dir_path: &Path, noshow: bool) {
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        if let Err(e) = search_in_file(keyword, file_path, noshow) {
            if !noshow {
                eprintln!("Error with file {}: {}", file_path.display(), e);
            }
        }
    }
}

fn search_in_file(keyword: &str, file_path: &Path, noshow: bool) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    for (line_num, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line) => {
                if let Some(highlighted) = highlight_keyword(&line, keyword) {
                    println!(
                        "{}:{} {}",
                        file_path.display().to_string().blue().bold(),
                        (line_num + 1).to_string().yellow().bold(),
                        highlighted
                    );
                }
            }
            Err(_) => {
                if !noshow {
                    eprintln!(
                        "Error with file {}: unreadable content",
                        file_path.display()
                    );
                }
                break;
            }
        }
    }

    Ok(())
}

fn highlight_keyword(line: &str, keyword: &str) -> Option<String> {
    let regex = Regex::new(keyword).ok()?;
    if regex.is_match(line) {
        Some(line.replace(keyword, &keyword.red().bold().to_string()))
    } else {
        None
    }
}
