#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;

use std::process::Command;
use std::collections::HashMap;
use std::fmt;


pub enum Profiler {
    PerfStat {
        task_clock: Option<f64>,
        context_switches: Option<f64>,
        cpu_migrations: Option<f64>,
        page_faults: Option<f64>,
        cycles: Option<f64>,
        instructions: Option<f64>,
        branches: Option<f64>,
        branch_misses: Option<f64>,
        seconds: Option<f64>,
        l1_dcache_loads: Option<f64>,
        l1_dcache_load_misses: Option<f64>,
        llc_loads: Option<f64>,
        llc_load_misses: Option<f64>,
    },
}


impl fmt::Display for Profiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Profiler::PerfStat { task_clock,
                                 context_switches,
                                 cpu_migrations,
                                 page_faults,
                                 cycles,
                                 instructions,
                                 branches,
                                 branch_misses,
                                 l1_dcache_loads,
                                 l1_dcache_load_misses,
                                 llc_loads,
                                 llc_load_misses,
                                 seconds } => {
                write!(f,
                       "\t\x1b[32mTask Clock\x1b[0m...{:.1} \t\t \x1b[32mContext \
                        Switches\x1b[0m...{:.1}\n\t\x1b[32mCPU Migrations\x1b[0m...{:.1} \t\t \
                        \x1b[32mPage Faults\x1b[0m...{:.1}\n\t\x1b[32mCycles\x1b[0m...{:.1} \t\t \
                        \x1b[32mInstructions\x1b[0m...{:.1}\n\t\x1b[32mBranches\x1b[0m...{:.1} \
                        \t\t \x1b[32mBranch Misses\x1b[0m...{:.1}\n\t\x1b[32ml1-dcache \
                        Loads\x1b[0m...{:.1} \t\t \x1b[32ml1-dcache Load \
                        Misses\x1b[0m...{:.1}\n\t\x1b[32mllc Loads\x1b[0m...{:.1} \t\t \
                        \x1b[32mllc Load Misses\x1b[0m...{:.1}\n\t\x1b[32mSeconds\x1b[0m...{:.3}\n",
                       task_clock.unwrap_or(std::f64::NAN),
                       context_switches.unwrap_or(std::f64::NAN),
                       cpu_migrations.unwrap_or(std::f64::NAN),
                       page_faults.unwrap_or(std::f64::NAN),
                       cycles.unwrap_or(std::f64::NAN),
                       instructions.unwrap_or(std::f64::NAN),
                       branches.unwrap_or(std::f64::NAN),
                       branch_misses.unwrap_or(std::f64::NAN),
                       l1_dcache_loads.unwrap_or(std::f64::NAN),
                       l1_dcache_load_misses.unwrap_or(std::f64::NAN),
                       llc_loads.unwrap_or(std::f64::NAN),
                       llc_load_misses.unwrap_or(std::f64::NAN),
                       seconds.unwrap_or(std::f64::NAN))
            }
        }


    }
}

pub trait Parser {
    fn cli(&self, binary: &str) -> String;
    fn parse(&self, output: &str) -> Profiler;
}


impl Profiler {
    pub fn new_perf() -> Profiler {
        Profiler::PerfStat {
            task_clock: None,
            context_switches: None,
            cpu_migrations: None,
            page_faults: None,
            cycles: None,
            instructions: None,
            branches: None,
            branch_misses: None,
            seconds: None,
            l1_dcache_loads: None,
            l1_dcache_load_misses: None,
            llc_loads: None,
            llc_load_misses: None,
        }
    }
}


impl Parser for Profiler {
    fn cli(&self, binary: &str) -> String {
        match *self {
            Profiler::PerfStat { .. } => {
                let perf_stat_output = Command::new("perf")
                                           .arg("stat")
                                           .arg(binary)
                                           .output()
                                           .unwrap_or_else(|e| panic!("failed {}", e));
                String::from_utf8(perf_stat_output.stderr).expect("cli error")

            }
        }

    }

    fn parse(&self, perf_stat_output: &str) -> Profiler {
        match *self {
            Profiler::PerfStat { .. } => {
                let out: Vec<&str> = perf_stat_output.split("\n").collect();
                let mut z = out[3..].to_owned();
                z.retain(|&x| {
                    !x.contains("<not supported>") & !x.contains("panicked") &
                    !x.contains("failed to read") & !x.contains("Performance")
                });
                let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|\d+(\.\d+)?");
                let re1 = regex!(r" [a-zA-Z]+-?[a-zA-Z]+?-?[a-zA-Z]+");

                let mut words: Vec<&str> = Vec::new();
                let mut numbers: Vec<f64> = Vec::new();

                for text in z.iter() {
                    if let Some(s) = re.find(text) {
                        let start = s.0 as usize;
                        let end = s.1 as usize;
                        unsafe {
                            let s = text.slice_unchecked(start, end);
                            numbers.push(s.trim()
                                          .replace(",", "")
                                          .parse::<f64>()
                                          .ok()
                                          .expect("f64 error"))
                        }
                    }

                }

                for text in z.iter() {
                    if let Some(s) = re1.find(text) {
                        let start = s.0 as usize;
                        let end = s.1 as usize;
                        unsafe {
                            let word = text.slice_unchecked(start, end).trim();
                            words.push(word)
                        }
                    }

                }

                let mut h = HashMap::new();
                for (&x, &y) in words.iter().zip(numbers.iter()) {
                    h.insert(x, y);
                }

                Profiler::PerfStat {
                    task_clock: h.get("task-clock").cloned(),
                    context_switches: h.get("context-switches").cloned(),
                    cpu_migrations: h.get("cpu-migrations").cloned(),
                    page_faults: h.get("page-faults").cloned(),
                    cycles: h.get("cycles").cloned(),
                    instructions: h.get("instructions").cloned(),
                    branches: h.get("branches").cloned(),
                    branch_misses: h.get("branch-misses").cloned(),
                    seconds: h.get("seconds").cloned(),
                    l1_dcache_loads: h.get("l1_dcache_loads").cloned(),
                    l1_dcache_load_misses: h.get("l1_dcache_load_misses").cloned(),
                    llc_loads: h.get("llc_loads").cloned(),
                    llc_load_misses: h.get("llc_load_misses").cloned(),
                }

            }

        }
    }
}
