# Resuman

Resuman is a command-line program that manages resumes written in Rust.

Note that resuman is supposed to be a simple program that manages resumes. It is not meant to be a full-fledged resume management system.

Advanced interface for resuman is still under development.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Add Resuman to Path

Run `cargo install --path .` to install the `resuman` command-line program.

## Usage

Run `cargo run -- --help` to see the help content for the `resuman` command-line program.

Alternatively, check out the [CommandLineHelp.md](CommandLineHelp.md) file.

## Development

### Generate Help Content

Run `cargo run -- --markdown-help > CommandLineHelp.md` to generate the help content for the `resuman` command-line program.

### Run Tests

No tests have been written yet. But when they are, run `cargo test` to run the tests.

Open a pull request to contribute to the project.
