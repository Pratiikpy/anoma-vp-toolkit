use anyhow::{Context, Result};
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn run(name: &str) -> Result<()> {
    let current_dir = env::current_dir()?;
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists.", name);
    }

    // Find and copy the template directory
    let exe_path = env::current_exe()?;
    let mut template_path = PathBuf::from(exe_path.parent().unwrap());
    template_path.push("templates/default_vp");

    if !template_path.exists() {
        template_path = PathBuf::from("templates/default_vp");
    }

    copy(&template_path, &current_dir, &CopyOptions::new())
        .context(format!("Failed to copy template from {:?}", template_path))?;

    std::fs::rename(current_dir.join("default_vp"), &target_dir)
        .context("Failed to rename template directory")?;

    // --- NEW: Dynamically update the package name ---
    let cargo_toml_path = target_dir.join("Cargo.toml");
    let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)
        .context("Failed to read the new project's Cargo.toml")?;

    cargo_toml_content = cargo_toml_content.replace(
        "name = \"my-anoma-vp\"",
        &format!("name = \"{}\"", name)
    );

    fs::write(&cargo_toml_path, cargo_toml_content)
        .context("Failed to write updated Cargo.toml")?;
    // --- End of new code ---

    Ok(())
} 