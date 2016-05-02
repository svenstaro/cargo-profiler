#![feature(plugin)]
#![plugin(regex_macros)]

extern crate clap;
extern crate regex;
extern crate profiler;
#[macro_use]
extern crate colorify;
use clap::{Arg, App};
use profiler::{Profiler, Parser};
use std::path::Path;
use std::process::Command;


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
                               .help("binary you want to profile"))
                      .arg(Arg::with_name("profiler")
                               .long("profiler")
                               .value_name("PROFILER")
                               .help("what profiler you want to use"))
                      .get_matches();

    // read binary argument, make sure it exists in the filesystem
    let binary = matches.value_of("binary").expect("failed to get argument binary");

    assert!(Path::new(binary).exists(),
            "That binary doesn't exist. Enter a valid path.");

    // read profiler argument
    let profiler = matches.value_of("profiler").expect("failed to get argument profiler");

    // initialize profiler based on argument
    let p = match profiler {
        "perf" => Profiler::new_perf(),
        "cachegrind" => Profiler::new_cachegrind(),
        "callgrind" => Profiler::new_callgrind(),
        _ => panic!("That profiler doesn't exist. Choose between perf, callgrind, and cachegrind."),

    };
    let path = binary.split("/").collect::<Vec<_>>();
    let name = path[path.len() - 1];
    println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36m{}...",
             name,
             profiler);
    // get the profiler output
    let output = p.cli(binary);

    // parse the output into struct
    let parsed = p.parse(&output);

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
