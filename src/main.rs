#![feature(plugin)]
#![plugin(regex_macros)]

extern crate clap;
extern crate regex;
extern crate profiler;
use clap::{Arg, App, SubCommand};
use profiler::{Profiler, Parser, Metric};
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


    // initialize variables to default ones
    let mut p = Profiler::new_cachegrind();
    let mut n = "all";
    let mut sort_metric = Metric :: NAN;
    let mut profiler = "none";
    let mut binary = "";

    // re-assign variables if subcommand is callgrind
    if let Some(matches) = matches.subcommand_matches("callgrind") {
        profiler = "callgrind";
        p = Profiler::new_callgrind();
        // read binary argument, make sure it exists in the filesystem
        binary = matches.value_of("binary").unwrap();

        assert!(Path::new(binary).exists(),
                "That binary doesn't exist. Enter a valid path.");


        if matches.is_present("n") {
            n = matches.value_of("n").unwrap();
        }

    }

    // re-assign variables if subcommand is cachegrind
    if let Some(matches) = matches.subcommand_matches("cachegrind") {
        profiler = "cachegrind";
        p = Profiler::new_cachegrind();

        binary = matches.value_of("binary").unwrap();

        assert!(Path::new(binary).exists(),
                "That binary doesn't exist. Enter a valid path.");

        if matches.is_present("n") {
            n = matches.value_of("n").unwrap();
        }
        if matches.is_present("sort") {
            sort_metric = match matches.value_of("sort").unwrap(){
                "ir" => Metric::Ir,
                 "i1mr"=>  Metric::I1mr,
                "ilmr"=> Metric::Ilmr  ,
                "dr"=>  Metric::Dr ,
                 "d1mr"=>  Metric::D1mr,
                "dlmr" =>  Metric::Dlmr,
                "dw"=>  Metric::Dw,
                 "d1mw"=> Metric::D1mw,
                 "dlmw"=> Metric::Dlmw,
                 _ => panic!("sort metric not valid")
            }
        }
    }



    // get the name of the binary from the binary argument
    let path = binary.split("/").collect::<Vec<_>>();
    let name = path[path.len() - 1];
    println!("\nProfiling \x1b[1;36m{} \x1b[0mwith \x1b[1;36m{}...",
             name,
             profiler);

    // get the profiler output
    let output = p.cli(binary);

    // parse the output into struct
    let parsed = p.parse(&output, n, sort_metric);

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
