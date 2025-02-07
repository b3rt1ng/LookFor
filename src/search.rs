use crate::utils::highlight_keywords;
use crate::writer::MultiWriter;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self};
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn search_in_file(
    keywords: &[String],
    file_path: &Path,
    noshow: bool,
    maxsize: usize,
    writer: &mut MultiWriter,
    keyword_counts: &mut HashMap<String, usize>,
    regex_counts: &mut Vec<usize>,
    regex_list: &[Regex],
    running: Arc<AtomicBool>,
    analyzed_files: &mut usize,
) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let file_size_mb = metadata.len() as usize / (1024 * 1024);

    if maxsize > 0 && file_size_mb > maxsize {
        if noshow {
            writeln!(
            writer,
            "{}: file exceeds {} MB, skipping",
            file_path.display().to_string().blue().bold(),
            maxsize
            )
            .unwrap();
        }
        return Ok(());
    }

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut found_any = false;

    for (line_num, line_result) in reader.lines().enumerate() {
        if !running.load(Ordering::SeqCst) {
            return Ok(());
        }
        match line_result {
            Ok(line) => {
                let mut matches = Vec::new();
                if !running.load(Ordering::SeqCst) {
                    return Ok(());
                }
                if let Some(highlighted) = highlight_keywords(&line, keywords) {
                    matches.push(highlighted);
                    for keyword in keywords {
                        if line.contains(keyword) {
                            if let Some(count) = keyword_counts.get_mut(keyword) {
                                *count += 1;
                            }
                        }
                    }
                }
                if !running.load(Ordering::SeqCst) {
                    return Ok(());
                }
                for (i, regex) in regex_list.iter().enumerate() {
                    for cap in regex.find_iter(&line) {
                        matches.push(format!("{}", cap.as_str().green().bold()));
                        regex_counts[i] += 1;
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
                if noshow {
                    eprintln!(
                        "UTF-8 read error in file {}, switching to binary mode.",
                        file_path.display()
                    );
                }
                found_any |= search_binary_content(
                    file_path,
                    keywords,
                    regex_list,
                    regex_counts,
                    writer,
                    keyword_counts,
                );
                break;
            }
        }
    }
    if !found_any && noshow {
        eprintln!("No keywords found in the file {}", file_path.display());
    }
    *analyzed_files += 1;
    Ok(())
}

fn search_binary_content(
    file_path: &Path,
    keywords: &[String],
    regex_list: &[Regex],
    regex_counts: &mut Vec<usize>,
    writer: &mut dyn Write,
    keyword_counts: &mut HashMap<String, usize>,
) -> bool {
    let content = match fs::read(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading binary file {}: {}", file_path.display(), e);
            return false;
        }
    };

    let printable_regex = Regex::new(r"[ -~]{4,}").unwrap();
    let content_str = String::from_utf8_lossy(&content);
    let mut found_any = false;
    for sequence in printable_regex.find_iter(&content_str) {
        let line = sequence.as_str();
        let mut matches = Vec::new();
        if let Some(highlighted) = highlight_keywords(line, keywords) {
            matches.push(highlighted);
            for keyword in keywords {
                let count = line.matches(keyword).count();
                *keyword_counts.entry(keyword.clone()).or_insert(0) += count;
            }
        }
        for (i, regex) in regex_list.iter().enumerate() {
            for cap in regex.find_iter(line) {
                matches.push(format!("{}", cap.as_str().green().bold()));
                regex_counts[i] += 1;
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
    regex_counts: &mut Vec<usize>,
    regex_list: &[Regex],
    omit_extensions: &Option<Vec<String>>,
    running: Arc<AtomicBool>,
    analyzed_files: &mut usize,
) {
    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if !running.load(Ordering::SeqCst) {
            return;
        }
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if ext_str.eq_ignore_ascii_case("log") || ext_str.eq_ignore_ascii_case("tmp") {
                    if !noshow {
                        eprintln!(
                            "Skipping file with omitted extension: {}",
                            file_path.display()
                        );
                    }
                    continue;
                }
                if let Some(omit_exts) = omit_extensions {
                    if omit_exts
                        .iter()
                        .any(|ext| ext.eq_ignore_ascii_case(ext_str))
                    {
                        if !noshow {
                            eprintln!(
                                "Skipping file with user-omitted extension: {}",
                                file_path.display()
                            );
                        }
                        continue;
                    }
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
            regex_counts,
            regex_list,
            running.clone(),
            analyzed_files,
        ) {
            if !noshow {
                eprintln!("Error with file {}: {}", file_path.display(), e);
            }
        }
    }
}
