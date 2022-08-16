mod maven;
mod sort_pom;

#[derive(Debug, clap::Subcommand)]
pub enum Args {
    SortPom(sort_pom::Args),
}

pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::SortPom(args) => sort_pom::main(args),
    }
}
