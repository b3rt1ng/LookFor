use colored::*;
use regex::Regex;

pub fn highlight_keywords(line: &str, keywords: &[String]) -> Option<String> {
    let mut highlighted = line.to_string();
    let mut found = false;

    for keyword in keywords {
        let regex = Regex::new(keyword).ok()?;
        if regex.is_match(&highlighted) {
            found = true;
            highlighted = highlighted.replace(
                keyword,
                &keyword.red().bold().to_string(),
            );
        }
    }

    if found {
        Some(highlighted)
    } else {
        None
    }
}
