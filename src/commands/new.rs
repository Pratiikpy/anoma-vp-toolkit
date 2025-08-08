use anyhow::{Context, Result};
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::PathBuf;

pub fn run(name: &str) -> Result<()> {
    let current_dir = env::current_dir()?;
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists.", name);
    }

    // For a packaged binary, the templates dir might be relative to the executable
    let exe_path = env::current_exe()?;
    let mut template_path = PathBuf::from(exe_path.parent().unwrap());
    template_path.push("templates/default_vp");

    // Fallback for local development `cargo run`
    if !template_path.exists() {
        template_path = PathBuf::from("templates/default_vp");
    }

    copy(&template_path, &current_dir, &CopyOptions::new())
        .context(format!("Failed to copy template from {:?}", template_path))?;

    std::fs::rename(current_dir.join("default_vp"), &target_dir)
        .context("Failed to rename template directory")?;

    Ok(())
} 