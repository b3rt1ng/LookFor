use clap::Parser;
use std::path::PathBuf;

/// Arguments for LookFor
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Keywords to search for (comma-separated)
    #[clap(short = 'f', long = "find")]
    pub find: Option<String>, // Rendez cet argument optionnel

    /// Directory or file to search in
    #[clap(short = 'p', long = "path", default_value = ".")]
    pub path: PathBuf,

    /// Show more information
    #[clap(long)]
    pub show: bool,

    /// Maximum file size to analyze in MB
    #[clap(short = 'm', long = "maxsize", default_value = "0")]
    pub maxsize: usize,

    /// Output file for results
    #[clap(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Omit certain file types (extensions) separated by commas
    #[clap(short = 'e', long = "omit", value_delimiter = ',')]
    pub omit: Option<Vec<String>>,

    /// Regex patterns to match words
    #[arg(short = 'r', long = "regex", value_delimiter = ',')]
    pub regex: Option<Vec<String>>,

    /// Positional argument for the search keyword
    #[clap()]
    pub positional_find: Option<String>,
}
