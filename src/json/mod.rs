mod sort_config;
mod sort_policy;

#[derive(Debug, clap::Subcommand)]
pub enum Args {
    SortConfig(sort_config::Args),
    SortPolicy(sort_policy::Args),
}

pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::SortConfig(args) => sort_config::main(args),
        Args::SortPolicy(args) => sort_policy::main(args),
    }
}
