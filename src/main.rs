mod json;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(about, version)]
enum Args {
    #[clap(subcommand)]
    Json(json::Args),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args {
        Args::Json(args) => json::main(args),
    }
}
