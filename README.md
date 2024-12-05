# Lookfor

**Lookfor** is a simple CLI tool written in Rust to search for keywords within files and directories, with support for color output and error handling.

## Features

- Search recursively in directories
- Highlight search results with colors
- Handle files that may not be readable (with an option to hide error messages, --noshow to hide those)

## Installation

To compile and use `lookfor`, you need to have Rust installed.

1. Clone this repository:
   ```bash
   git clone https://github.com/b3rt1ng/LookFor
   cd lookfor
   ```

2. Try it
   ```bash
   cargo run -- "bash" /path/to/look/for
   ```
3. Make it runable
   ```bash
   cargo build --release
   sudo cp target/release/lookfor /usr/local/bin/

   #run it
   lookfor "bash" . --noshow
   ```

# Usage
```
  -f, --find <FIND>        Keywords to search for (comma-separated)
  -p, --path <PATH>        Directory or file to search in
  -n, --noshow             Do not display errors for unreadable files
  -m, --maxsize <MAXSIZE>  Maximum file size to analyze in MB [default: 0]
  -o, --output <OUTPUT>    Output file for results
  -h, --help               Print help
  -V, --version            Print version
```
### Examples:
```bash
lookfor -f bash,file -p . -m 200
# search for the words file and bash within the current directory and with a max file size of 200MO
```

## TODO
- multiple paths
- not case sensitive argument
