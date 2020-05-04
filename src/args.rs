use structopt::StructOpt;
use escargot;

#[derive(StructOpt, Clone, Debug)]
#[structopt(
    bin_name = "cargo",
    name = "cargo-profiler",
    author,
    about,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp],
)]
pub enum Args {
    Profiler {
        #[structopt(subcommand)]
        command: Command,
    }
}

#[derive(StructOpt, Clone, Debug)]
pub enum Command {
    Profile {
        /// Name of the bin target to profile
        #[structopt(long, name = "NAME")]
        bin: Option<String>,

        /// Build artifacts in release mode, with optimizations
        #[structopt(long)]
        release: bool,
    },
    Stat {
        /// Name of the bin target to profile
        #[structopt(long, name = "NAME")]
        bin: Option<String>,

        /// Build artifacts in release mode, with optimizations
        #[structopt(long)]
        release: bool,
    },
}
