use crate::utils::highlight_keywords;
use crate::writer::MultiWriter;
use std::fs::{self};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;
use colored::*;
use std::io::Write;
use std::collections::HashMap;

pub fn search_in_file(
    keywords: &[String],
    file_path: &Path,
    noshow: bool,
    maxsize: usize,
    writer: &mut MultiWriter,
    keyword_counts: &mut HashMap<String, usize>,
    regex_count: &mut usize, // Counter for regex matches
    regex: Option<&Regex>,   // Regex pattern
) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let file_size_mb = metadata.len() as usize / (1024 * 1024);

    if maxsize > 0 && file_size_mb > maxsize {
        writeln!(
            writer,
            "{}: file exceeds {} MB, skipping",
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
                // Collect matches for keywords and regex
                let mut matches = Vec::new();

                // Highlight and count keyword matches
                if let Some(highlighted) = highlight_keywords(&line, keywords) {
                    for keyword in keywords {
                        if line.contains(keyword) {
                            if let Some(count) = keyword_counts.get_mut(keyword) {
                                *count += 1;
                            }
                        }
                    }
                    matches.push(highlighted);
                }

                // Highlight and count regex matches
                if let Some(regex) = regex {
                    for cap in regex.find_iter(&line) {
                        matches.push(format!("{}", cap.as_str().green().bold()));
                        *regex_count += 1;
                    }
                }

                if !matches.is_empty() {
                    writeln!(
                        writer,
                        "{}:{} {}",
                        file_path.display().to_string().blue().bold(),
                        (line_num + 1).to_string().yellow().bold(),
                        matches.join(", ")
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
                found_any |= search_binary_content(file_path, keywords, regex, regex_count, writer);
                break;
            }
        }
    }

    if !found_any && !noshow {
        eprintln!("No keywords found in the file {}", file_path.display());
    }

    Ok(())
}

fn search_binary_content(
    file_path: &Path,
    keywords: &[String],
    regex: Option<&Regex>,
    regex_count: &mut usize, // Counter for regex matches
    writer: &mut dyn Write,
) -> bool {
    let content = match fs::read(file_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let printable_regex = Regex::new(r"[ -~]{4,}").unwrap();
    let content_str = String::from_utf8_lossy(&content);

    let mut found_any = false;

    for sequence in printable_regex.find_iter(&content_str) {
        let line = sequence.as_str();

        let mut matches = Vec::new();

        // Highlight and count keyword matches
        if let Some(highlighted) = highlight_keywords(line, keywords) {
            matches.push(highlighted);
        }

        // Highlight and count regex matches
        if let Some(regex) = regex {
            for cap in regex.find_iter(line) {
                matches.push(format!("{}", cap.as_str().green().bold()));
                *regex_count += 1;
            }
        }

        if !matches.is_empty() {
            writeln!(
                writer,
                "{}: {}",
                file_path.display().to_string().blue().bold(),
                matches.join(", ")
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
    keyword_counts: &mut HashMap<String, usize>,
    regex_count: &mut usize, // Counter for regex matches
    omit_extensions: &[String],
    regex: Option<&Regex>, // Regex pattern
) {
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();

        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if omit_extensions.contains(&ext_str.to_lowercase()) {
                    if !noshow {
                        eprintln!(
                            "Skipping file with omitted extension: {}",
                            file_path.display()
                        );
                    }
                    continue;
                }
            }
        }

        if let Err(e) = search_in_file(
            keywords,
            file_path,
            noshow,
            maxsize,
            writer,
            keyword_counts,
            regex_count, // Pass regex match counter
            regex,       // Pass regex
        ) {
            if !noshow {
                eprintln!("Error with file {}: {}", file_path.display(), e);
            }
        }
    }
}
