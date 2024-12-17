use clap::Parser;
use std::path::PathBuf;

/// Arguments for LookFor
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Keywords to search for (comma-separated)
    #[clap(short = 'f', long = "find")]
    pub find: String,

    /// Directory or file to search in
    #[clap(short = 'p', long = "path", default_value = ".")]
    pub path: PathBuf,

    /// Do not display errors for unreadable files
    #[clap(short = 'n', long = "noshow")]
    pub noshow: bool,

    /// Maximum file size to analyze in MB
    #[clap(short = 'm', long = "maxsize", default_value = "0")]
    pub maxsize: usize,

    /// Output file for results
    #[clap(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Omit certain file types (extensions) separated by commas
    #[arg(short = 'e', long = "omit", value_delimiter = ',')]
    pub omit: Option<Vec<String>>,
}
