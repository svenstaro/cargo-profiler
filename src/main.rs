#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]
extern crate clap;
extern crate regex;
extern crate profiler;
#[macro_use]
extern crate colorify;
use clap::{Arg, App};
use profiler::{Profiler, Parser};
#[cfg(all(unix, target_os = "linux"))]
fn main() {

    let matches = App::new("cargo-profiler")
                      .version("1.0")
                      .author("Suchin Gururangan")
                      .about("Profile your app")
                      .arg(Arg::with_name("binary")
                               .long("bin")
                               .value_name("BINARY")
                               .help("binary you want to profile"))
                      .arg(Arg::with_name("profiler")
                               .long("profiler")
                               .value_name("PROFILER")
                               .help("what profiler you want to use"))
                      .get_matches();
    let binary = matches.value_of("binary").expect("failed to get argument binary");
    let profiler = matches.value_of("profiler").expect("failed to get argument profiler");
    let p = match profiler {
        "perf" => Profiler::new_perf(),
        "cachegrind" => Profiler::new_cachegrind(),
        "callgrind" => Profiler::new_callgrind(),
        _ => panic!("That profiler doesn't exist. Choose between perf, callgrind, and cachegrind."),

    };
    let output = p.cli(binary);
    let parsed = p.parse(&output);
    match profiler {
        "perf" => printc!(white: "\nPerf Stat Output:\n\n"),
        "cachegrind" => printc!(white: "\nCacheGrind Output:\n\n"),
        "callgrind" => printc!(white : "\nCallGrind Output: \n\n"),
        _ => panic!("That profiler doesn't exist. Choose between perf, callgrind, and cachegrind."),
    }

    println!("{}", parsed)


}
