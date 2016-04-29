#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;

use std::process::Command;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Perf {
    task_clock : Option<f64>,
    context_switches : Option<f64>,
    cpu_migrations : Option<f64>,
    page_faults : Option<f64>,
    cycles : Option<f64>,
    instructions : Option<f64>,
    branches : Option<f64>,
    branch_misses : Option<f64>,
    seconds : Option<f64>,
    L1_dcache_loads : Option<f64>,
    L1_dcache_load_misses : Option<f64>,
    LLC_loads : Option<f64>,
    LLC_load_misses : Option<f64>

}

impl fmt::Display for Perf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "\t\x1b[32mTask Clock\x1b[0m...{:.1} \t\t \x1b[32mContext Switches\x1b[0m...{:.1}\n\
            \t\x1b[32mCPU Migrations\x1b[0m...{:.1} \t\t \x1b[32mPage Faults\x1b[0m...{:.1}\n\
            \t\x1b[32mCycles\x1b[0m...{:.1} \t\t\t \x1b[32mInstructions\x1b[0m...{:.1}\n\
            \t\x1b[32mBranches\x1b[0m...{:.1} \t\t\t \x1b[32mBranch Misses\x1b[0m...{:.1}\n\
            \t\x1b[32mL1-dcache Loads\x1b[0m...{:.1} \t\t \x1b[32mL1-dcache Load Misses\x1b[0m...{:.1}\n\
            \t\x1b[32mLLC Loads\x1b[0m...{:.1} \t\t \x1b[32mLLC Load Misses\x1b[0m...{:.1}\n\
            \t\x1b[32mSeconds\x1b[0m...{:.3}\n\
            ",
                    self.task_clock.unwrap_or(std::f64::NAN),
                    self.context_switches.unwrap_or(std::f64::NAN),
                    self.cpu_migrations.unwrap_or(std::f64::NAN),
                    self.page_faults.unwrap_or(std::f64::NAN),
                    self.cycles.unwrap_or(std::f64::NAN),
                    self.instructions.unwrap_or(std::f64::NAN),
                    self.branches.unwrap_or(std::f64::NAN),
                    self.branch_misses.unwrap_or(std::f64::NAN),
                    self.L1_dcache_loads.unwrap_or(std::f64::NAN),
                    self.L1_dcache_load_misses.unwrap_or(std::f64::NAN),
                    self.LLC_loads.unwrap_or(std::f64::NAN),
                    self.LLC_load_misses.unwrap_or(std::f64::NAN),
                    self.seconds.unwrap_or(std::f64::NAN))


    }
}

pub trait Parser {
    fn cli(&self, binary : &str) -> String;
    fn parse(&self, perf_stat_output : &str) -> Perf;
}


impl Perf {
    pub fn new() -> Perf {
        Perf {
            task_clock : None,
            context_switches : None,
            cpu_migrations : None,
            page_faults : None,
            cycles : None,
            instructions : None,
            branches : None,
            branch_misses : None,
            seconds : None,
            L1_dcache_loads : None,
            L1_dcache_load_misses : None,
            LLC_loads : None,
            LLC_load_misses : None
        }

    }
}
impl Parser for Perf {

    fn cli(&self, binary : &str) -> String {
        let perf_stat_output = Command::new("perf")
                                            .arg("stat")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| {panic!("failed {}", e)});
        String::from_utf8(perf_stat_output.stderr).expect("cli error")

    }

    fn parse(&self, perf_stat_output : &str)-> Perf {

        let out : Vec<&str> = perf_stat_output.split("\n").collect();
        let mut z = out[3..].to_owned();
        z.retain(|&x| !x.contains("<not supported>")& !x.contains("panicked") & !x.contains("failed to read") & !x.contains("Performance"));
        let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|\d+(\.\d+)?");
        let re1 = regex!(r" [a-zA-Z]+-?[a-zA-Z]+?-?[a-zA-Z]+");

        let mut words : Vec <&str>= Vec :: new();
        let mut numbers : Vec<f64> = Vec :: new();

        for text in z.iter() {
            if let Some(s) =  re.find(text) {
                let start = s.0 as usize;
                let end = s.1 as usize;
                unsafe{
                let s = text.slice_unchecked(start, end);
                numbers.push(
                        s
                        .trim()
                        .replace(",","")
                        .parse::<f64>()
                        .ok()
                        .expect("f64 error")
                    )
                }
            }

        }

        for text in z.iter() {
            if let Some(s) =  re1.find(text) {
                let start = s.0 as usize;
                let end = s.1 as usize;
                unsafe{
                let word = text.slice_unchecked(start, end).trim();
                words.push(word)
                }
            }

        }

        let mut h = HashMap :: new();
        for (&x,&y) in words.iter().zip(numbers.iter()) {
            h.insert(x,y);
        }

        Perf {
            task_clock : h.get("task-clock").cloned(),
            context_switches : h.get("context-switches").cloned(),
            cpu_migrations :h.get("cpu-migrations").cloned(),
            page_faults : h.get("page-faults").cloned(),
            cycles : h.get("cycles").cloned(),
            instructions : h.get("instructions").cloned(),
            branches : h.get("branches").cloned(),
            branch_misses : h.get("branch-misses").cloned(),
            seconds : h.get("seconds").cloned(),
            L1_dcache_loads : h.get("L1_dcache_loads").cloned(),
            L1_dcache_load_misses : h.get("L1_dcache_load_misses").cloned(),
            LLC_loads : h.get("LLC_loads").cloned(),
            LLC_load_misses : h.get("LLC_load_misses").cloned()
        }

    }
}
