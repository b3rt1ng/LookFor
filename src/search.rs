use crate::utils::highlight_keywords;
use crate::writer::MultiWriter;
use std::fs::{self};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;
use colored::*;
use std::io::Write;

pub fn search_in_file(
    keywords: &[String],
    file_path: &Path,
    noshow: bool,
    maxsize: usize,
    writer: &mut MultiWriter,
) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let file_size_mb = metadata.len() as usize / (1024 * 1024);

    if maxsize > 0 && file_size_mb > maxsize {
        writeln!(
            writer,
            "{}: fichier dépassant {} Mo, on passe",
            file_path.display().to_string().blue().bold(),
            maxsize
        )
        .unwrap();
        return Ok(());
    }

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut found_any = false;

    for (line_num, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line) => {
                if let Some(highlighted) = highlight_keywords(&line, keywords) {
                    writeln!(
                        writer,
                        "{}:{} {}",
                        file_path.display().to_string().blue().bold(),
                        (line_num + 1).to_string().yellow().bold(),
                        highlighted
                    )
                    .unwrap();
                    found_any = true;
                }
            }
            Err(_) => {
                if !noshow {
                    eprintln!(
                        "UTF-8 read error in file {}, switching to binary mode.",
                        file_path.display()
                    );
                }
                found_any |= search_binary_content(file_path, keywords, writer);
                break;
            }
        }
    }

    if !found_any && !noshow {
        eprintln!("No keywords found in the file {}", file_path.display());
    }

    Ok(())
}

fn search_binary_content(file_path: &Path, keywords: &[String], writer: &mut dyn Write) -> bool {
    let content = match fs::read(file_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let printable_regex = Regex::new(r"[ -~]{4,}").unwrap();
    let content_str = String::from_utf8_lossy(&content);

    let mut found_any = false;

    for sequence in printable_regex.find_iter(&content_str) {
        let line = sequence.as_str();
        if let Some(highlighted) = highlight_keywords(line, keywords) {
            writeln!(
                writer,
                "{}: {}",
                file_path.display().to_string().blue().bold(),
                highlighted
            )
            .unwrap();
            found_any = true;
        }
    }

    found_any
}

pub fn search_in_directory(
    keywords: &[String],
    dir_path: &Path,
    noshow: bool,
    maxsize: usize,
    writer: &mut MultiWriter,
) {
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        if let Err(e) = search_in_file(keywords, file_path, noshow, maxsize, writer) {
            if !noshow {
                eprintln!("Error with file {}: {}", file_path.display(), e);
            }
        }
    }
}
