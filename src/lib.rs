#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;

use std::process::Command;
use std::collections::HashMap;
use std::fmt;


pub enum Profiler<'a> {
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

    CacheGrind {
        i_refs: Option<f64>,
        i1_misses: Option<f64>,
        lli_misses: Option<f64>,
        i1_miss_rate: Option<f64>,
        lli_miss_rate: Option<f64>,
        d_refs: Option<f64>,
        d1_misses: Option<f64>,
        d1_miss_rate: Option<f64>,
        lld_misses: Option<f64>,
        lld_miss_rate: Option<f64>,
        ll_refs: Option<f64>,
        ll_misses: Option<f64>,
        ll_miss_rate: Option<f64>,
    },
    CallGrind {
        instruction_0: Option<f64>,
        funct_0: Option<&'a str>,
        instruction_1: Option<f64>,
        funct_1: Option<&'a str>,
        instruction_2: Option<f64>,
        funct_2: Option<&'a str>,
        instruction_3: Option<f64>,
        funct_3: Option<&'a str>,
        instruction_4: Option<f64>,
        funct_4: Option<&'a str>,
        instruction_5: Option<f64>,
        funct_5: Option<&'a str>,
        instruction_6: Option<f64>,
        funct_6: Option<&'a str>,
        instruction_7: Option<f64>,
        funct_7: Option<&'a str>,
        instruction_8: Option<f64>,
        funct_8: Option<&'a str>,
        instruction_9: Option<f64>,
        funct_9: Option<&'a str>,
    },
}


impl<'a> fmt::Display for Profiler<'a> {
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
                       "\x1b[32mTask Clock\x1b[0m...{:.1} \n\x1b[32mContext \
                        Switches\x1b[0m...{:.1}\n\x1b[32mCPU Migrations\x1b[0m...{:.1} \n\
                        \x1b[32mPage Faults\x1b[0m...{:.1}\n\x1b[32mCycles\x1b[0m...{:.1} \n\
                        \x1b[32mInstructions\x1b[0m...{:.1}\n\x1b[32mBranches\x1b[0m...{:.1} \
                        \n\x1b[32mBranch Misses\x1b[0m...{:.1}\n\x1b[32ml1-dcache \
                        Loads\x1b[0m...{:.1} \n\x1b[32ml1-dcache Load \
                        Misses\x1b[0m...{:.1}\n\x1b[32mllc Loads\x1b[0m...{:.1} \n\
                        \x1b[32mllc Load Misses\x1b[0m...{:.1}\n\x1b[32mSeconds\x1b[0m...{:.3}\n",
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
            Profiler::CacheGrind { i_refs,
                                   i1_misses,
                                   lli_misses,
                                   i1_miss_rate,
                                   lli_miss_rate,
                                   d_refs,
                                   d1_misses,
                                   d1_miss_rate,
                                   lld_misses,
                                   lld_miss_rate,
                                   ll_refs,
                                   ll_misses,
                                   ll_miss_rate } => {
                write!(f,
                       "\x1b[32mTotal I-Cache References\x1b[0m...{:.1} \n\x1b[32mL1 I-Cache \
                        Misses\x1b[0m...{:.1}\n\x1b[32mL1 I-Cache Miss Rate\x1b[0m...{:.1} \n\
                        \x1b[32mL2 I-Cache Misses\x1b[0m...{:.1}\n\x1b[32mL2 I-Cache Miss \
                        Rate\x1b[0m...{:.1} \n\x1b[32mTotal D-Cache \
                        References\x1b[0m...{:.1}\n\x1b[32mL1 D-Cache Misses\x1b[0m...{:.1} \
                        \n\x1b[32mL1 D-Cache Miss Rate\x1b[0m...{:.1}\n\x1b[32mL2 D-Cache \
                        Misses\x1b[0m...{:.1} \n\x1b[32mL2 D-Cache Miss \
                        Rate\x1b[0m...{:.1}\n\x1b[32mTotal L2-Cache References\x1b[0m...{:.1} \
                        \n\x1b[32mL2 Cache Misses\x1b[0m...{:.1}\n\x1b[32mL2 Miss \
                        Rate\x1b[0m...{:.3}\n",
                       i_refs.unwrap_or(std::f64::NAN),
                       i1_misses.unwrap_or(std::f64::NAN),
                       i1_miss_rate.unwrap_or(std::f64::NAN),
                       lli_misses.unwrap_or(std::f64::NAN),
                       lli_miss_rate.unwrap_or(std::f64::NAN),
                       d_refs.unwrap_or(std::f64::NAN),
                       d1_misses.unwrap_or(std::f64::NAN),
                       d1_miss_rate.unwrap_or(std::f64::NAN),
                       lld_misses.unwrap_or(std::f64::NAN),
                       lld_miss_rate.unwrap_or(std::f64::NAN),
                       ll_refs.unwrap_or(std::f64::NAN),
                       ll_misses.unwrap_or(std::f64::NAN),
                       ll_miss_rate.unwrap_or(std::f64::NAN))
            }
            Profiler::CallGrind { instruction_0,
                                  funct_0,
                                  instruction_1,
                                  funct_1,
                                  instruction_2,
                                  funct_2,
                                  instruction_3,
                                  funct_3,
                                  instruction_4,
                                  funct_4,
                                  instruction_5,
                                  funct_5,
                                  instruction_6,
                                  funct_6,
                                  instruction_7,
                                  funct_7,
                                  instruction_8,
                                  funct_8,
                                  instruction_9,
                                  funct_9 } => {
                write!(f,
                       "\x1b[\n32m{:.2}\x1b[0m...{} \n\x1b[\n32m{:.2}\x1b[0m...{} \n\
                        \x1b[\n32m{:.2}\x1b[0m...{} \n\x1b[\n32m{:.2}\x1b[0m...{} \n\
                         \x1b[\n32m{:.2}\x1b[0m...{} \n\x1b[\n32m{:.2}\x1b[0m...{} \n\
                         \x1b[\n32m{:.2}\x1b[0m...{} \n\x1b[\n32m{:.2}\x1b[0m...{} \n\
                         \x1b[\n32m{:.2}\x1b[0m...{} \n\x1b[\n32m{:.2}\x1b[0m...{} \n\n",
                       instruction_0.unwrap_or(std::f64::NAN),
                       funct_0.unwrap_or("NAN"),
                       instruction_1.unwrap_or(std::f64::NAN),
                       funct_1.unwrap_or("NAN"),
                       instruction_2.unwrap_or(std::f64::NAN),
                       funct_2.unwrap_or("NAN"),
                       instruction_3.unwrap_or(std::f64::NAN),
                       funct_3.unwrap_or("NAN"),
                       instruction_4.unwrap_or(std::f64::NAN),
                       funct_4.unwrap_or("NAN"),
                       instruction_5.unwrap_or(std::f64::NAN),
                       funct_5.unwrap_or("NAN"),
                       instruction_6.unwrap_or(std::f64::NAN),
                       funct_6.unwrap_or("NAN"),
                       instruction_7.unwrap_or(std::f64::NAN),
                       funct_7.unwrap_or("NAN"),
                       instruction_8.unwrap_or(std::f64::NAN),
                       funct_8.unwrap_or("NAN"),
                       instruction_9.unwrap_or(std::f64::NAN),
                       funct_9.unwrap_or("NAN"),
                   )
            }

        }




    }
}

pub trait Parser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str) -> Profiler;
}


impl<'a> Profiler<'a> {
    pub fn new_perf() -> Profiler<'a> {
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
    pub fn new_cachegrind() -> Profiler<'a> {
        Profiler::CacheGrind {
            i_refs: None,
            i1_misses: None,
            lli_misses: None,
            i1_miss_rate: None,
            lli_miss_rate: None,
            d_refs: None,
            d1_misses: None,
            d1_miss_rate: None,
            lld_misses: None,
            lld_miss_rate: None,
            ll_refs: None,
            ll_misses: None,
            ll_miss_rate: None,
        }
    }

    pub fn new_callgrind() -> Profiler<'a> {
        Profiler::CallGrind {
            instruction_0: None,
            funct_0: None,
            instruction_1: None,
            funct_1: None,
            instruction_2: None,
            funct_2: None,
            instruction_3: None,
            funct_3: None,
            instruction_4: None,
            funct_4: None,
            instruction_5: None,
            funct_5: None,
            instruction_6: None,
            funct_6: None,
            instruction_7: None,
            funct_7: None,
            instruction_8: None,
            funct_8: None,
            instruction_9: None,
            funct_9: None,
        }
    }
}


impl<'a> Parser for Profiler<'a> {
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
            Profiler::CacheGrind { .. } => {
                let cachegrind_output = Command::new("valgrind")
                                            .arg("--tool=cachegrind")
                                            .arg("--cachegrind-out-file=cachegrind.out")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| panic!("failed {}", e));
                String::from_utf8(cachegrind_output.stderr).expect("cli error")
            }
            Profiler::CallGrind { .. } => {
                Command::new("valgrind")
                    .arg("--tool=callgrind")
                    .arg("--callgrind-out-file=callgrind.out")
                    .arg(binary)
                    .output()
                    .unwrap_or_else(|e| panic!("failed {}", e));
                let cachegrind_output = Command::new("callgrind_annotate")
                                            .arg("callgrind.out")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| panic!("failed {}", e));
                String::from_utf8(cachegrind_output.stdout).expect("cli error")
            }
        }

    }

    fn parse<'b>(&'b self, output: &'b str) -> Profiler {
        match *self {
            Profiler::PerfStat { .. } => {
                let out: Vec<&str> = output.split("\n").collect();
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
            Profiler::CacheGrind { .. } => {
                let mut out: Vec<&str> = output.split("\n").collect();
                out.retain(|&x| x.contains("=="));
                let z = out[6..].to_owned();
                let r = regex!(r"==\d+==");
                let mut zr: Vec<String> = Vec::new();
                for text in z.iter() {
                    zr.push(r.replace_all(text, ""));

                }
                let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|[^\w\d{1}]&\d+(\.\d+)?|\d*\.\d+");
                let re1 = regex!(r"[a-zA-Z\d{1}?]+\s*\t*[a-zA-Z\d{1}?]+\s*\t*[a-zA-Z]+:");

                let mut words: Vec<String> = Vec::new();
                let mut numbers: Vec<f64> = Vec::new();

                for text in zr.iter() {
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


                for text in zr.iter() {
                    if let Some(s) = re1.find(text) {
                        let start = s.0 as usize;
                        let end = s.1 as usize;
                        unsafe {

                            let d = text.slice_unchecked(start, end)
                                        .trim()
                                        .to_lowercase()
                                        .replace(":", "")
                                        .split(" ")
                                        .collect::<Vec<_>>()
                                        .join("");

                            words.push(d);
                        }
                    }

                }

                let mut h = HashMap::new();
                for (x, &y) in words.iter().zip(numbers.iter()) {
                    h.insert(x.as_ref(), y);
                }


                Profiler::CacheGrind {
                    i_refs: h.get("irefs").cloned(),
                    i1_misses: h.get("i1misses").cloned(),
                    i1_miss_rate: h.get("i1missrate").cloned(),
                    lli_misses: h.get("llimisses").cloned(),
                    lli_miss_rate: h.get("llimissrate").cloned(),
                    d_refs: h.get("drefs").cloned(),
                    d1_misses: h.get("d1misses").cloned(),
                    d1_miss_rate: h.get("d1missrate").cloned(),
                    lld_misses: h.get("lldmisses").cloned(),
                    lld_miss_rate: h.get("lldmissrate").cloned(),
                    ll_refs: h.get("llrefs").cloned(),
                    ll_misses: h.get("llmisses").cloned(),
                    ll_miss_rate: h.get("llmissrate").cloned(),
                }

            }

            Profiler::CallGrind { .. } => {
                let out = output.split("\n").collect::<Vec<_>>();
                let z = out[25..out.len()].to_owned();

                let mut words: Vec<&str> = Vec::new();
                let mut numbers = Vec::new();
                for text in z.iter() {
                    let text = text.trim();
                    let elems = text.split("  ").collect::<Vec<_>>();
                    if let Ok(s) = elems[0]
                                       .trim()
                                       .replace(",", "")
                                       .parse::<f64>() {
                        numbers.push(s);
                    }
                    if elems.len() > 1 {
                        words.push(elems[1])
                    }
                }

                let total_instructions = numbers.iter().fold(0.0, |a, b| a + b);
                let numbers = numbers.iter().map(|x| x / total_instructions).collect::<Vec<f64>>();

                Profiler::CallGrind {
                    instruction_0: numbers.get(0).cloned(),
                    funct_0: words.get(0).cloned(),
                    instruction_1: numbers.get(1).cloned(),
                    funct_1: words.get(1).cloned(),
                    instruction_2: numbers.get(2).cloned(),
                    funct_2: words.get(2).cloned(),
                    instruction_3: numbers.get(3).cloned(),
                    funct_3: words.get(3).cloned(),
                    instruction_4: numbers.get(4).cloned(),
                    funct_4: words.get(4).cloned(),
                    instruction_5: numbers.get(5).cloned(),
                    funct_5: words.get(5).cloned(),
                    instruction_6: numbers.get(6).cloned(),
                    funct_6: words.get(6).cloned(),
                    instruction_7: numbers.get(7).cloned(),
                    funct_7: words.get(7).cloned(),
                    instruction_8: numbers.get(8).cloned(),
                    funct_8: words.get(8).cloned(),
                    instruction_9: numbers.get(9).cloned(),
                    funct_9: words.get(9).cloned(),
                }
            }

        }
    }
}
