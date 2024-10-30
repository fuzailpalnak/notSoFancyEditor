# NotSoFancyEditor
![NotSoFancyEditor](https://github.com/user-attachments/assets/b49a4e7f-f8bc-4040-95dc-eede8c818341)

This is a simple terminal based text editor built in Rust, allowing users to type, delete, and save text files directly from the terminal. This project serves as a practical introduction to Rust's features and capabilities, particularly focusing on memory safety and efficient resource management.

## Features

- **Type and Edit Text:** Users can input and edit text directly in the terminal.
- **Delete Text:** Effortlessly delete characters using backspace.
- **Save Files:** Save the current text to a file on disk.

## Technology Stack

- **Language:** Rust
- **Library:** [Crossterm](https://crates.io/crates/crossterm) for cross-platform terminal manipulation.

## Getting Started

### Prerequisites

- Install Rust: If you haven’t already, you can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Clone the Repository

```bash
git clone https://github.com/yourusername/simple-text-editor.git
cd simple-text-editor
```

### Build and Run

To build and run the project, use the following commands:

```bash
cargo build
cargo run
```

### Usage

1. Run the editor using `cargo run`.
2. Type your text in the terminal.
3. Use the backspace key to delete characters and arrows to navigate.
4. To save your work type Ctrl+S
5. To exit type Ctrl+X

