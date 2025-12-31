# Project Overview

This is a simple command-line calculator application written in Rust. It takes two numbers and an operator as arguments and performs the specified arithmetic operation.

## Technologies Used

*   Rust

## Building and Running

To build the project, navigate to the root directory and run:

```bash
cargo build --release
```

To run the calculator, use the following format:

```bash
cargo run <first_number> <operator> <second_number>
```

**Examples:**

```bash
cargo run 10 + 5
cargo run 20 x 4
cargo run 100 / 25
```

## Development Conventions

*   **Language:** Rust
*   **Error Handling:** The current implementation uses `unwrap()` for argument parsing, which can cause the program to panic if invalid arguments are provided. Future improvements could involve more robust error handling.
*   **License:** Apache License 2.0
