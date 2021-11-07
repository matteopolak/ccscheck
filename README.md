**All paths mentioned will be relative to `/opt/CyberPatriot` on Linux, and `C:/CyberPatriot` on Windows.**

## Requirements
* [Rust](https://www.rust-lang.org/tools/install) 1.56.0 or higher

## Building
* Clone the repository: `git clone https://github.com/matteopolak/ccscheck.git`
* Enter the directory: `cd ccscheck`
* Build the project: `cargo build --release`

## Required files & folders
* **hashes** folder, with the file names as answer hashes and no contents
* Build the project and rename the binary to **CCSCheck[.exe]**

## Getting hashes
1. Pipe a file (with one answer per line) into CCSGen, [located here](https://github.com/matteopolak/ccsgen)