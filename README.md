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

## TODO
- output generation
- multiple words
- multiple paths
