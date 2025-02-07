# Lookfor

**Lookfor** is a simple CLI tool written in Rust to search for keywords within files and directories, with support for color output and error handling.

## Features

- Search recursively in directories
- Highlight search results with colors
- Handle files that may not be readable (with an option to hide error messages, --noshow to hide those)
- You can use the -r argument to find an additional regex

## Installation

To compile and use `lookfor`, you need to have Rust installed.

1. Clone this repository:
   ```bash
   git clone https://github.com/b3rt1ng/LookFor
   cd lookfor
   ```

2. Try it
   ```bash
   cargo run -- -f "bash"
   ```
3. Make it runable
   ```bash
   cargo build --release
   sudo cp target/release/lookfor /usr/local/bin/

   #run it
   lookfor -f bash -p . --noshow
   ```

# Usage
```
  -f, --find <FIND>        Keywords to search for (comma-separated)
  -p, --path <PATH>        Directory or file to search in [default: .]
      --show               Show more information
  -m, --maxsize <MAXSIZE>  Maximum file size to analyze in MB [default: 0]
  -o, --output <OUTPUT>    Output file for results
  -e, --omit <OMIT>        Omit certain file types (extensions) separated by commas
  -r, --regex <REGEX>      Regex patterns to match words
  -h, --help               Print help
```
### Examples:
```bash
lookfor word
# most simple way to use it, it will look for "word" in your current subdirectories 
```
```bash
lookfor -f bash,file -m 200
# search for the words file and bash within the current directory and with a max file size of 200MO
```
```bash
lookfor -f bash,file -r "\b(?:\d{1,3}\.){3}\d{1,3}\b"
# search for the words file and bash within the current as well as anything that looks like an IP
```

## TODO
- file type handling (such as csr, github logs etc...)
