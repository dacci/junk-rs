mod json;
mod xml;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(about, version)]
enum Args {
    #[command(subcommand)]
    Json(json::Args),

    #[command(subcommand)]
    Xml(xml::Args),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args {
        Args::Json(args) => json::main(args),
        Args::Xml(args) => xml::main(args),
    }
}
