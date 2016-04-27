extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main(){
    
    let matches = App::new("cargo-profiler")
                    .version("1.0")
                    .author("Suchin Gururangan")
                    .about("Profile your app")
                    .arg(Arg::with_name("binary")
                        .long("bin")
                        .value_name("BINARY")
                        .help("binary you want to profile")
                    ).get_matches();
    let binary = matches.value_of("binary").expect("failed to get argument");

    // get cachegrind output
    let cachegrind_output = Command::new("valgrind")
                                        .arg("--tool=cachegrind")
                                        .arg(binary)
                                        .output()
                                        .unwrap_or_else(|e| {panic!("failed {}", e)});

    // get perf stat output
    let perf_stat_output = Command::new("perf")
                                        .arg("stat")
                                        .arg(binary)
                                        .output()
                                        .unwrap_or_else(|e| {panic!("failed {}", e)});

    let perf_stat_output = String::from_utf8_lossy(&perf_stat_output.stderr);
    let cachegrind_output = String::from_utf8_lossy(&cachegrind_output.stderr);

    println!("{:?}", perf_stat_output);
    println!("{:?}", cachegrind_output);

}
