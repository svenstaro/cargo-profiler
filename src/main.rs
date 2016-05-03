#![feature(plugin)]
#![plugin(regex_macros)]

extern crate clap;
extern crate regex;
extern crate profiler;
#[macro_use]
extern crate colorify;
use clap::{Arg, App, SubCommand};
use profiler::{Profiler, Parser};
use std::path::Path;
use std::process::Command;
use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);


#[cfg(all(unix, target_os = "linux"))]
fn main() {

    // create profiler application
    let matches = App::new("cargo-profiler")
                      .version("1.0")
                      .author("Suchin Gururangan")
                      .about("Profile your binaries")
                      .arg(Arg::with_name("binary")
                               .long("bin")
                               .value_name("BINARY")
                               .required(true)
                               .help("binary you want to profile"))
                      .subcommand(SubCommand::with_name("callgrind")
                                      .about("gets callgrind features")
                                      .version("1.0")
                                      .author("Suchin Gururangan")
                                      .arg(Arg::with_name("n")
                                               .short("n")
                                               .value_name("NUMBER")
                                               .takes_value(true)
                                               .help("number of functions you want")))
                      .subcommand(SubCommand::with_name("cachegrind")
                                      .about("gets cachegrind features")
                                      .version("1.0")
                                      .author("Suchin Gururangan"))
                      .subcommand(SubCommand::with_name("perf")
                                      .about("gets perf features")
                                      .version("1.0")
                                      .author("Suchin Gururangan"))
                      .get_matches();

    // read binary argument, make sure it exists in the filesystem
    let binary = matches.value_of("binary").unwrap();

    assert!(Path::new(binary).exists(),
            "That binary doesn't exist. Enter a valid path.");

    let mut p = Profiler::new_cachegrind();
    let mut n = "";
    let mut profiler = "";
    if let Some(matches) = matches.subcommand_matches("callgrind") {
        profiler = "callgrind";
        p = Profiler::new_callgrind();
        if matches.is_present("n") {
            n = matches.value_of("n").unwrap();
        } else {
            n = "all";
        }
    }

    if let Some(_) = matches.subcommand_matches("cachegrind") {
        profiler = "cachegrind";
        p = Profiler::new_cachegrind();
    }

    if let Some(_) = matches.subcommand_matches("perf") {
        profiler = "perf";
        p = Profiler::new_perf();
    }


    let path = binary.split("/").collect::<Vec<_>>();
    let name = path[path.len() - 1];
    println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36m{}...",
             name,
             profiler);
    // get the profiler output
    let output = p.cli(binary);

    // parse the output into struct
    let parsed = p.parse(&output, n);

    // pretty-print
    println!("{}", parsed);

    // remove files generated while profiling
    Command::new("rm")
        .arg("cachegrind.out")
        .output()
        .unwrap_or_else(|e| panic!("failed {}", e));

    Command::new("rm")
        .arg("callgrind.out")
        .output()
        .unwrap_or_else(|e| panic!("failed {}", e));
}
