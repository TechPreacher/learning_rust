# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Build all examples: `cargo build`
- Build specific example: `cargo build --bin arrays_vectors`
- Run specific example: `cargo run --bin arrays_vectors`
- Check: `cargo check`
- Test: `cargo test` (for a single test: `cargo test test_name`)
- Lint: `cargo clippy`

## Project Structure
- Each example has its own binary in `src/bin/`
- Binaries defined in `Cargo.toml` using `[[bin]]` sections
- Each file must contain its own `main()` function

## Code Style Guidelines
- Use 4 spaces for indentation
- Add explicit type annotations for clarity: `let a: i32 = 10;`
- Functions use snake_case naming: `fn add_numbers()`
- Variables use snake_case naming: `let is_even`
- Use trailing semicolons except for expressions returning values
- Comment code with line comments (`//`) to explain concepts
- No external dependencies - standard library only
- Use idiomatic Rust patterns (iterators, closures) when appropriate
- Prefer immutable variables (use `mut` only when necessary)