pub mod argparse;
pub mod cargo;
pub mod display;
pub mod err;
pub mod parse;
pub mod profiler;

use crate::argparse::{get_binary, get_num, get_profiler, get_sort_metric};
use crate::cargo::build_binary;
use crate::err::ProfError;
use crate::parse::cachegrind::CacheGrindParser;
use crate::parse::callgrind::CallGrindParser;
use crate::profiler::Profiler;
use clap::{App, AppSettings, Arg, SubCommand};
use std::ffi::OsStr;
use std::process;
use std::process::Command;

// macro to try something, but print custom error message and exit upon error.
macro_rules! try_or_exit {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    };
}

fn main() {
    let _ = real_main();
}

// #[cfg(all(unix, any(target_os = "linux", target_os = "macos")))]
#[cfg(unix)]
fn real_main() -> Result<(), ProfError> {
    // create binary path argument
    let binary_arg = Arg::with_name("binary")
        .long("bin")
        .value_name("BINARY")
        .required(false)
        .help("binary you want to profile");

    // create binary arguments positional args (aka, everything after a '--')
    let binargs_arg = Arg::with_name("binargs")
        .multiple(true)
        .value_name("BIN_ARGS")
        .required(false)
        .help("arguments to the binary when executed");

    // create release argument
    let release = Arg::with_name("release")
        .long("release")
        .required(false)
        .help("whether binary should be built in release mode");

    // create function count argument
    let fn_count_arg = Arg::with_name("n")
        .short("n")
        .value_name("NUMBER")
        .takes_value(true)
        .help("number of functions you want");

    // create sort metric argument
    let sort_arg = Arg::with_name("sort")
        .long("sort")
        .value_name("SORT")
        .takes_value(true)
        .help("metric you want to sort by");

    // keep output files
    let keep_arg = Arg::with_name("keep")
        .long("keep")
        .required(false)
        .help("keep profiler output files");


    // create callgrind subcommand
    let callgrind = SubCommand::with_name("callgrind")
        .about("gets callgrind features")
        .version("1.0")
        .author("Suchin Gururangan")
        .arg(release.clone())
        .arg(binary_arg.clone())
        .arg(binargs_arg.clone())
        .arg(fn_count_arg.clone())
        .arg(keep_arg.clone());

    // create cachegrind subcommand
    let cachegrind = SubCommand::with_name("cachegrind")
        .about("gets cachegrind features")
        .version("1.0")
        .author("Suchin Gururangan")
        .arg(release)
        .arg(binary_arg)
        .arg(binargs_arg.clone())
        .arg(fn_count_arg)
        .arg(sort_arg)
        .arg(keep_arg);

    // create profiler subcommand
    let profiler = SubCommand::with_name("profiler")
        .about("gets callgrind features")
        .version("1.0")
        .author("Suchin Gururangan")
        .subcommand(callgrind)
        .subcommand(cachegrind);

    // create profiler application
    let matches = App::new("cargo-profiler")
        .bin_name("cargo")
        .settings(&[AppSettings::SubcommandRequired])
        .version("1.0")
        .author("Suchin Gururangan")
        .about("Profile your binaries")
        .subcommand(profiler)
        .get_matches();

    // parse arguments from cli call
    let (m, profiler) = try_or_exit!(get_profiler(&matches));
    let binary = {
        if m.is_present("binary") {
            try_or_exit!(get_binary(&m)).to_string()
        } else if m.is_present("release") {
            try_or_exit!(build_binary(true))
        } else {
            try_or_exit!(build_binary(false))
        }
    };

    let binary_name = binary.split('/').collect::<Vec<&str>>().pop().unwrap_or("");
    let binargs: Vec<&OsStr> = match m.values_of_os("binargs") {
        None => vec![],
        Some(raw) => raw.collect(),
    };

    let num = try_or_exit!(get_num(&m));
    let sort_metric = try_or_exit!(get_sort_metric(&m));

    match profiler {
        Profiler::CallGrind { .. } => println!(
            "\n\x1b[1;33mProfiling \x1b[1;0m{} \x1b[0mwith callgrind\x1b[0m...",
            binary_name
        ),
        Profiler::CacheGrind { .. } => println!(
            "\n\x1b[1;33mProfiling \x1b[1;0m{} \x1b[0mwith cachegrind\x1b[0m...",
            binary_name
        ),
    };

    // get the profiler output
    let output = match profiler {
        Profiler::CallGrind { .. } => profiler.callgrind_cli(&binary, &binargs)?,
        Profiler::CacheGrind { .. } => profiler.cachegrind_cli(&binary, &binargs)?,
    };

    // parse the output into struct
    let parsed = match profiler {
        Profiler::CallGrind { .. } => try_or_exit!(profiler.callgrind_parse(&output, num)),
        Profiler::CacheGrind { .. } => {
            try_or_exit!(profiler.cachegrind_parse(&output, num, sort_metric))
        }
    };

    // pretty-print
    println!("{}", parsed);

    if !m.is_present("keep") {
        // remove files generated while profiling
        Command::new("rm").arg("cachegrind.out").output()?;

        Command::new("rm").arg("callgrind.out").output()?;
    }

    Ok(())
}
