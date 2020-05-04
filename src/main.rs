use crate::args::Args;
use anyhow::Result;
use structopt::StructOpt;

mod args;

fn main() -> Result<()> {
    let args = Args::from_args();

    dbg!(args);

    Ok(())
}
