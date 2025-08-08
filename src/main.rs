use clap::Parser;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    commands::dispatch(cli.command)
} 