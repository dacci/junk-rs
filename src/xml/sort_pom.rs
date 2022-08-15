use anyhow::{Error, Result};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct Args {
    path: PathBuf,
}

pub fn main(args: Args) -> Result<()> {
    let f = File::open(&args.path)?;
    let pom: super::maven::Model = yaserde::de::from_reader(f).map_err(Error::msg)?;

    let mut f = File::create(&args.path)?;
    yaserde::ser::serialize_with_writer(
        &pom,
        &mut f,
        &yaserde::ser::Config {
            perform_indent: true,
            write_document_declaration: true,
            indent_string: None,
        },
    )
    .map_err(Error::msg)?;
    f.write_all(b"\n")?;

    Ok(())
}
