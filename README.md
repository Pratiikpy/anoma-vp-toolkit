# Anoma Validity Predicate Toolkit (`anoma-vp-toolkit`)

A CLI toolkit to generate, compile, and test Validity Predicates (VPs) for the Anoma network, making development faster and easier.

This tool helps bootstrap new VP projects, abstracting away the boilerplate and letting you focus on the core logic of your state transition rules.

## Features

- **`vp-toolkit new`**: Generate a new VP project from a ready-made Rust template.
- **`vp-toolkit build`**: Compile your VP to the required `wasm32-unknown-unknown` target. (Coming Soon)
- **`vp-toolkit test`**: Test your compiled Wasm VP against local JSON intent files for rapid, offline development. (Coming Soon)

## Installation

```bash
# Installation instructions will be added here once the project is published.
```

## Usage

### 1. Create a new Validity Predicate

This will create a new directory with a boilerplate Rust project, ready for your custom logic.

```bash
anoma-vp-toolkit new my-first-vp
```

This creates the following structure:

```
my-first-vp/
├── .gitignore
├── Cargo.toml
└── src/
    └── lib.rs
```

### 2. Add Your Logic

Navigate into the new directory (`cd my-first-vp`) and open `src/lib.rs` to implement your validation rules.

### 3. Build and Test (Coming Soon)

Once implemented, you will be able to easily compile and test your VP:

```bash
# Navigate into your VP's directory
cd my-first-vp

# Compile it to Wasm
anoma-vp-toolkit build

# Test it against an intent
anoma-vp-toolkit test --vp ./target/wasm32-unknown-unknown/release/my_first_vp.wasm --intent ./tests/valid_intent.json
```

## Contribution

This is a community tool and contributions are welcome! 