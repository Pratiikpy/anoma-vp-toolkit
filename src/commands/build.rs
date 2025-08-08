use anyhow::{Context, Result};
use colored::*;
use std::process::Command;
use std::env;

pub fn run() -> Result<()> {
    // 1. Check if we are in a Rust project directory
    let current_dir = env::current_dir()?;
    if !current_dir.join("Cargo.toml").exists() {
        anyhow::bail!(
            "No Cargo.toml found in the current directory. Please run this command from within a Validity Predicate project."
        );
    }

    println!("{} Compiling VP to Wasm...", "⏳".yellow());

    // 2. Execute the cargo build command
    let output = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .output()
        .context("Failed to execute cargo build command. Make sure you have the 'wasm32-unknown-unknown' target installed (`rustup target add wasm32-unknown-unknown`).")?;

    // 3. Check if the command was successful
    if output.status.success() {
        println!("{} Build successful!", "✅".green());
        
        // Find the project name from Cargo.toml to show the correct path
        let manifest = std::fs::read_to_string("Cargo.toml")?;
        let toml: toml::Value = toml::from_str(&manifest)?;
        let package_name = toml.get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("your_vp_name");

        let wasm_path = format!("target/wasm32-unknown-unknown/release/{}.wasm", package_name.replace("-", "_"));
        
        println!("   Wasm file located at: {}", wasm_path.cyan());

    } else {
        // If it failed, print the error output from cargo
        println!("{} Build failed.", "❌".red());
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        anyhow::bail!("Cargo build process failed.");
    }

    Ok(())
} 