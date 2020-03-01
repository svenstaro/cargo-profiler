pub mod argparse;
pub mod args;
pub mod cargo;
pub mod display;
pub mod err;
pub mod parse;
pub mod profiler;

use crate::argparse::{get_binary, get_profiler, get_sort_metric};
use crate::cargo::build_binary;
use crate::err::ProfError;
use crate::parse::cachegrind::CachegrindParser;
use crate::parse::callgrind::CallgrindParser;
use crate::args::ProfilerType;
use clap::{App, AppSettings, Arg, SubCommand};
use std::ffi::OsStr;
use std::process;
use std::process::Command;
use structopt::StructOpt;
use anyhow::Result;

fn main() -> Result<()> {
    let args = args::CargoProfilerConfig::from_args();

    // The way cargo extension programs are meant to be written requires us to always have a
    // variant here that is the only one that'll ever be usd.
    let profiler_type = if let args::CargoProfilerConfig::Profiler { profiler_type } = args {
        profiler_type
    } else {
        unreachable!();
    };

    match profiler_type {
        ProfilerType::Callgrind { binary_name, .. } => println!(
            "\n\x1b[1;33mProfiling \x1b[1;0m{} \x1b[0mwith callgrind\x1b[0m...",
            binary_name
        ),
        ProfilerType::Cachegrind { binary_name, .. } => println!(
            "\n\x1b[1;33mProfiling \x1b[1;0m{} \x1b[0mwith cachegrind\x1b[0m...",
            binary_name
        ),
    };

    // // parse arguments from cli call
    // let (m, profiler) = try_or_exit!(get_profiler(&matches));
    // let binary = {
    //     if m.is_present("binary") {
    //         try_or_exit!(get_binary(&m)).to_string()
    //     } else if m.is_present("release") {
    //         try_or_exit!(build_binary(true))
    //     } else {
    //         try_or_exit!(build_binary(false))
    //     }
    // };
    //
    // let binary_name = binary.split('/').collect::<Vec<&str>>().pop().unwrap_or("");
    // let binargs: Vec<&OsStr> = match m.values_of_os("binargs") {
    //     None => vec![],
    //     Some(raw) => raw.collect(),
    // };
    //
    // let num = try_or_exit!(get_num(&m));
    // let sort_metric = try_or_exit!(get_sort_metric(&m));
    //
    //
    // // get the profiler output
    // let output = match profiler {
    //     Profiler::CallGrind { .. } => profiler.callgrind_cli(&binary, &binargs)?,
    //     Profiler::CacheGrind { .. } => profiler.cachegrind_cli(&binary, &binargs)?,
    // };
    //
    // // parse the output into struct
    // let parsed = match profiler {
    //     Profiler::CallGrind { .. } => try_or_exit!(profiler.callgrind_parse(&output, num)),
    //     Profiler::CacheGrind { .. } => {
    //         try_or_exit!(profiler.cachegrind_parse(&output, num, sort_metric))
    //     }
    // };
    //
    //
    //
    // // pretty-print
    // println!("{}", parsed);
    //
    // if !args.keep {
    //     // remove files generated while profiling
    //     Command::new("rm").arg("cachegrind.out").output()?;
    //     Command::new("rm").arg("callgrind.out").output()?;
    // }

    Ok(())
}
