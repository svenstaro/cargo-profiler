use structopt::StructOpt;
use crate::parse::cachegrind::Metric;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "cargo-profiler",
    author,
    about,
    global_settings = &[structopt::clap::AppSettings::ColoredHelp]
)]
pub enum CargoProfilerConfig {
    Profiler {
        #[structopt(subcommand)]
        profiler_type: ProfilerType,
    }
}

#[derive(Debug, Clone, StructOpt)]
pub enum ProfilerType {
    /// Run callgrind
    Callgrind {
        /// Binary you want to profile
        #[structopt(name = "BIN", long)]
        binary: Option<String>,

        /// Arguments for the binary
        #[structopt(name = "ARG")]
        bin_args: Vec<String>,

        /// Build binary in release mode
        #[structopt(long)]
        release: bool,

        /// Number of functions you want
        #[structopt(short, default_value = "10000")]
        n_functions: u16,

        /// Keep profiler output files
        #[structopt(short, long)]
        keep: bool,
    },
    /// Run cachegrind
    Cachegrind {
        /// Binary you want to profile
        #[structopt(name = "BIN", long)]
        binary: Option<String>,

        /// Arguments for the binary
        #[structopt(name = "ARG")]
        bin_args: Vec<String>,

        /// Build binary in release mode
        #[structopt(long)]
        release: bool,

        /// Number of functions you want
        #[structopt(short, default_value = "10000")]
        n_functions: u16,

        /// Metric you want to sort by
        #[structopt(short, long, default_value = "NAN")]
        sort: Metric,

        /// Keep profiler output files
        #[structopt(short, long)]
        keep: bool,
    },
}
