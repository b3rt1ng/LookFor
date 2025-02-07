use colored::*;
use regex::Regex;

pub fn highlight_keywords(line: &str, keywords: &[String]) -> Option<String> {
    let mut highlighted = line.to_string();
    let mut found = false;

    for keyword in keywords {
        let word_boundary_pattern = format!(r"\b{}\b", regex::escape(keyword));
        let regex = Regex::new(&word_boundary_pattern).ok()?;
        
        if regex.is_match(&highlighted) {
            found = true;
            highlighted = regex.replace_all(&highlighted, |_: &regex::Captures| {
                keyword.red().bold().to_string()
            }).to_string();
        }
    }

    if found {
        Some(highlighted)
    } else {
        None
    }
}
