use clap::Subcommand;
use anyhow::Result;
use colored::*;

// Import the modules for each command
pub mod new;
pub mod build;
pub mod test;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Validity Predicate project from a template
    New {
        /// The name of the new VP project
        name: String,
    },
    /// Compile the VP project to Wasm
    Build,
    /// Test a compiled VP against a JSON intent file (coming soon)
    Test,
}

pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::New { name } => {
            new::run(&name)?;
            println!("\n✅ {}", format!("Success! Created new VP project '{}'", name).green());
            println!("   Run 'cd {}' to get started.", name);
        }
        Commands::Build => {
            println!("{}", "Build command coming soon!".yellow());
        }
        Commands::Test => {
            println!("{}", "Test command coming soon!".yellow());
        }
    }
    Ok(())
} 