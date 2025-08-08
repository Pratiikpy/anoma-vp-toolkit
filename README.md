# Anoma Validity Predicate Toolkit (`anoma-vp-toolkit`)

A CLI toolkit to generate, compile, and test Validity Predicates (VPs) for the Anoma network, making development faster and easier.

This tool provides a complete `new -> build -> test` workflow, abstracting away the boilerplate and letting you focus on the core logic of your state transition rules.

## Features

- **`vp-toolkit new <PROJECT_NAME>`**: Generates a new VP project from a ready-made Rust template.
- **`vp-toolkit build`**: Compiles your VP to the required `wasm32-unknown-unknown` target.
- **`vp-toolkit test`**: Executes your compiled Wasm VP in a local runtime against a JSON intent file for rapid, offline development.

## Installation

First, ensure you have the Rust toolchain and the Wasm target installed:
```bash
rustup target add wasm32-unknown-unknown
```

Then, install the toolkit directly from this repository:

```bash
cargo install --git https://github.com/Pratiikpy/anoma-opensource.git
```

## Usage

### 1. Create a New Validity Predicate

This will create a new directory with a boilerplate Rust project, ready for your custom logic.

```bash
anoma-vp-toolkit new my-first-vp
```

### 2. Add Your Logic & Create a Test Intent

Navigate into the new directory (`cd my-first-vp`) and open `src/lib.rs` to implement your validation rules.

Then, create a test intent file. For example, create `tests/valid_intent.json`:

```json
{
  "source": "acc1...",
  "target": "acc2...",
  "token": "NAM",
  "amount": 100
}
```

### 3. Build and Test Your VP

From within your `my-first-vp` directory, run the following commands:

```bash
# Compile your VP to Wasm
anoma-vp-toolkit build

# Test it against your intent file
anoma-vp-toolkit test --vp ./target/wasm32-unknown-unknown/release/my_first_vp.wasm --intent ./tests/valid_intent.json
```

## Contribution

This is a community tool and contributions are welcome! Feel free to open an issue or submit a pull request. 