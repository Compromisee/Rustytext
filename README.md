# bigtext

Render text as large box-drawing block letters in your terminal.

```
██████╗ ██╗ ██████╗ ████████╗███████╗██╗  ██╗████████╗
██╔══██╗██║██╔════╝ ╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝
██████╔╝██║██║  ███╗   ██║   █████╗   ╚███╔╝    ██║   
██╔══██╗██║██║   ██║   ██║   ██╔══╝   ██╔██╗    ██║   
██████╔╝██║╚██████╔╝   ██║   ███████╗██╔╝ ██╗   ██║   
╚═════╝ ╚═╝ ╚═════╝    ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝   
```
<sub> Don't worry, it fully works in terminal like in the screenshot, *no stripes* </sub>
## Requirements

- [Rust + Cargo](https://rustup.rs/)
- Windows: [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with **Desktop development with C++**

## Setup

Place `Cargo.toml` at the root and `bigtext.rs` as `src/main.rs`.

Your folder should look like:

```
Rustytext/
├── Cargo.toml
└── src/
    └── main.rs
```

## Usage

```bash
cargo run -- "YOUR TEXT HERE"
```

### Examples

```bash
cargo run -- "HELLO WORLD"
cargo run -- "LAN SHARE"
cargo run -- "RUST 2026"
cargo run -- "GAME OVER!"
```

Running with no arguments defaults to `HELLO`:

```bash
cargo run
```

## Supported Characters

| Category   | Characters                          |
|------------|-------------------------------------|
| Letters    | A–Z (automatically uppercased)      |
| Numbers    | 0–9                                 |
| Punctuation| `! ? . , - _ : /`                  |
| Space      | ` `                                 |

Unrecognised characters render as a placeholder block.

## Running Tests

```bash
cargo test
```

## Using as a Library

The `render()` function is public — you can use it in other Rust projects:

```rust
mod bigtext;

fn main() {
    println!("{}", bigtext::render("HELLO"));
}
```
