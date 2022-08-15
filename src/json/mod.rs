mod sort_config;

#[derive(Debug, clap::Subcommand)]
pub enum Args {
    SortConfig(sort_config::Args),
}

pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::SortConfig(args) => sort_config::main(args),
    }
}
