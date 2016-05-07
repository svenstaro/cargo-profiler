#![feature(plugin)]
#![plugin(regex_macros)]

extern crate clap;
extern crate regex;

pub mod profiler;
pub mod parse;
pub mod display;

use clap::{Arg, App, SubCommand};
use profiler::Profiler;
use parse::callgrind::CallGrindParser;
use parse::cachegrind::{CacheGrindParser, Metric};
use std::path::Path;
use std::process::Command;


#[cfg(all(unix, target_os = "linux"))]
fn main() {
    // create profiler application
    let matches = App::new("cargo-profiler")
                      .version("1.0")
                      .author("Suchin Gururangan")
                      .about("Profile your binaries")
                      .subcommand(SubCommand::with_name("callgrind")
                                      .about("gets callgrind features")
                                      .version("1.0")
                                      .author("Suchin Gururangan")
                                      .arg(Arg::with_name("binary")
                                               .long("bin")
                                               .value_name("BINARY")
                                               .required(true)
                                               .help("binary you want to profile"))
                                      .arg(Arg::with_name("n")
                                               .short("n")
                                               .value_name("NUMBER")
                                               .takes_value(true)
                                               .help("number of functions you want")))
                      .subcommand(SubCommand::with_name("cachegrind")
                                      .about("gets cachegrind features")
                                      .version("1.0")
                                      .author("Suchin Gururangan")
                                      .arg(Arg::with_name("binary")
                                               .long("bin")
                                               .value_name("BINARY")
                                               .required(true)
                                               .help("binary you want to profile"))
                                      .arg(Arg::with_name("n")
                                               .short("n")
                                               .value_name("NUMBER")
                                               .takes_value(true)
                                               .help("number of functions you want"))
                                      .arg(Arg::with_name("sort")
                                               .long("sort")
                                               .value_name("SORT")
                                               .takes_value(true)
                                               .help("metric you want to sort by")))
                      .get_matches();


    let (matches, profiler) = match matches.subcommand_matches("callgrind") {

        Some(matches) => (matches, Profiler::new_callgrind()),
        None => {
            match matches.subcommand_matches("cachegrind") {
                Some(matches) => (matches, Profiler::new_cachegrind()),
                None => panic!("Invalid profiler"),
            }
        }
    };




    // read binary argument, make sure it exists in the filesystem
    let binary = match matches.value_of("binary") {
        Some(z) => z,
        None => panic!("did not supply valid binary"),
    };

    assert!(Path::new(binary).exists(),
            "That binary doesn't exist. Enter a valid path.");


    let num = match matches.value_of("n").unwrap().parse::<usize>() {
        Ok(z) => z,
        Err(_) => panic!("did not supply valid number argument"),
    };

    let sort_metric = match matches.value_of("sort") {
        Some("ir") => Metric::Ir,
        Some("i1mr") => Metric::I1mr,
        Some("ilmr") => Metric::Ilmr,
        Some("dr") => Metric::Dr,
        Some("d1mr") => Metric::D1mr,
        Some("dlmr") => Metric::Dlmr,
        Some("dw") => Metric::Dw,
        Some("d1mw") => Metric::D1mw,
        Some("dlmw") => Metric::Dlmw,
        None => Metric::NAN,
        _ => panic!("sort metric not valid"),
    };

    // get the name of the binary from the binary argument
    let path = binary.split("/").collect::<Vec<_>>();
    let name = path[path.len() - 1];

    match profiler {
        Profiler::CallGrind { .. } => {
            println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36mcallgrind...",
                     name)
        }
        Profiler::CacheGrind { .. } => {
            println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36mcachegrind...",
                     name)
        }
    };


    // get the profiler output
    let output = match profiler {
        Profiler::CallGrind { .. } => profiler.callgrind_cli(binary),
        Profiler::CacheGrind { .. } => profiler.cachegrind_cli(binary),
    };
    // parse the output into struct
    let parsed = match profiler {
        Profiler::CallGrind { .. } => profiler.callgrind_parse(&output, num),
        Profiler::CacheGrind { .. } => profiler.cachegrind_parse(&output, num, sort_metric),
    };

    // pretty-print
    println!("{}", parsed);

    // remove files generated while profiling
    Command::new("rm")
        .arg("cachegrind.out")
        .output()
        .unwrap_or_else(|e| panic!("failed to remove {}", e));

    Command::new("rm")
        .arg("callgrind.out")
        .output()
        .unwrap_or_else(|e| panic!("failed to remove {}", e));
}
