#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;

use std::process::Command;
use std::collections::HashMap;


pub struct Perf {
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
}

pub struct CallGrind{
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
}

pub struct CacheGrind{
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
}

pub enum Profiler {
    CacheGrind{
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
}, CallGrind{
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
}, Perf{
    TaskClock : Option<f64>,
    ContextSwitches : Option<f64>,
    CPUMigrations : Option<f64>,
    PageFaults : Option<f64>,
    Cycles : Option<f64>,
    Instructions : Option<f64>,
    Branches : Option<f64>,
    BranchMisses : Option<f64>,
    Seconds : Option<f64>
} }


impl Perf {
    pub fn new() -> Self {
        Perf {
            TaskClock : None,
            ContextSwitches : None,
            CPUMigrations : None,
            PageFaults : None,
            Cycles : None,
            Instructions : None,
            Branches : None,
            BranchMisses : None,
            Seconds : None,
        }
    }

}
impl CallGrind {
    pub fn new() -> Self {
        CallGrind {
            TaskClock : None,
            ContextSwitches : None,
            CPUMigrations : None,
            PageFaults : None,
            Cycles : None,
            Instructions : None,
            Branches : None,
            BranchMisses : None,
            Seconds : None,
        }
    }

}
impl CacheGrind {
    pub fn new() -> Self {
        CacheGrind {
            TaskClock : None,
            ContextSwitches : None,
            CPUMigrations : None,
            PageFaults : None,
            Cycles : None,
            Instructions : None,
            Branches : None,
            BranchMisses : None,
            Seconds : None,
        }
    }

}

trait Parser {
    type Variant;
    fn cli(binary : &str) -> String;
    fn parse(perf_stat_output : String) -> Profiler;
}



impl Parser for Perf {
    type Variant = Perf;

    fn cli(binary : &str) -> String {
        let perf_stat_output = Command::new("perf")
                                            .arg("stat")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| {panic!("failed {}", e)});
        String::from_utf8(perf_stat_output.stderr).unwrap()

    }

    fn parse(perf_stat_output : String)-> Profiler {

        let out : Vec<&str> = perf_stat_output.split("\n").collect();
        // println!("{:?}", out);
        let mut z = out[3..].to_owned();
        z.retain(|&x| !x.contains("<not supported>"));
        let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|\d+(\.\d+)?");
        let re1 = regex!(r" [a-zA-Z]+-?[a-zA-Z]+-?[a-zA-Z]+");

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
                        .unwrap()
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
        // PerfOutput.
        for (&x,&y) in words.iter().zip(numbers.iter()) {
            h.insert(x,y);
        }

        Profiler::Perf {
            TaskClock : Some(*h.get("task-clock").unwrap()),
            ContextSwitches : Some(*h.get("context-switches").unwrap()),
            CPUMigrations : Some(*h.get("cpu-migrations").unwrap()),
            PageFaults : Some(*h.get("page-faults").unwrap()),
            Cycles : Some(*h.get("cycles").unwrap()),
            Instructions : Some(*h.get("instructions").unwrap()),
            Branches : Some(*h.get("branches").unwrap()),
            BranchMisses : Some(*h.get("branch-misses").unwrap()),
            Seconds : Some(*h.get("seconds").unwrap())
        }

    }
}


impl Parser for CallGrind {
    type Variant = CallGrind;

    fn cli(binary : &str) -> String {

        Command::new("valgrind")
        .arg("--tool=callgrind")
        .arg("--callgrind-out-file=new_callgrind.txt")
        .arg(binary)
        .output().unwrap_or_else(|e| {panic!("failed {}", e)});

        let callgrind_output = Command::new("callgrind_annotate")
                                            .arg("new_callgrind.txt")
                                            .output()
                                            .unwrap_or_else(|e| {panic!("failed {}", e)});


        String::from_utf8(callgrind_output.stdout).unwrap()
    }

    fn parse(perf_stat_output : String)-> Profiler {

        let out : Vec<&str> = perf_stat_output.split("\n").collect();
        // println!("{:?}", out);
        let mut z = out[3..].to_owned();
        z.retain(|&x| !x.contains("<not supported>"));
        let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|\d+(\.\d+)?");
        let re1 = regex!(r" [a-zA-Z]+-?[a-zA-Z]+-?[a-zA-Z]+");

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
                        .unwrap()
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
        // PerfOutput.
        for (&x,&y) in words.iter().zip(numbers.iter()) {
            h.insert(x,y);
        }

        Profiler::CallGrind {
            TaskClock : Some(*h.get("task-clock").unwrap()),
            ContextSwitches : Some(*h.get("context-switches").unwrap()),
            CPUMigrations : Some(*h.get("cpu-migrations").unwrap()),
            PageFaults : Some(*h.get("page-faults").unwrap()),
            Cycles : Some(*h.get("cycles").unwrap()),
            Instructions : Some(*h.get("instructions").unwrap()),
            Branches : Some(*h.get("branches").unwrap()),
            BranchMisses : Some(*h.get("branch-misses").unwrap()),
            Seconds : Some(*h.get("seconds").unwrap())
        }

    }
}

impl Parser for CacheGrind {
    type Variant = CacheGrind;

    fn cli(binary : &str) -> String {

        Command::new("valgrind")
        .arg("--tool=cachegrind")
        .arg("--cachegrind-out-file=new_cachegrind.txt")
        .arg(binary)
        .output().unwrap_or_else(|e| {panic!("failed {}", e)});

        let cachegrind_output = Command::new("cg_annotate")
                                            .arg("new_cachegrind.txt")
                                            .output()
                                            .unwrap_or_else(|e| {panic!("failed {}", e)});

        String::from_utf8(cachegrind_output.stdout).unwrap()
    }

    fn parse(perf_stat_output : String)-> Profiler {

        let out : Vec<&str> = perf_stat_output.split("\n").collect();
        // println!("{:?}", out);
        let mut z = out[3..].to_owned();
        z.retain(|&x| !x.contains("<not supported>"));
        let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|\d+(\.\d+)?");
        let re1 = regex!(r" [a-zA-Z]+-?[a-zA-Z]+-?[a-zA-Z]+");

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
                        .unwrap()
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
        // PerfOutput.
        for (&x,&y) in words.iter().zip(numbers.iter()) {
            h.insert(x,y);
        }

        Profiler::Perf {
            TaskClock : Some(*h.get("task-clock").unwrap()),
            ContextSwitches : Some(*h.get("context-switches").unwrap()),
            CPUMigrations : Some(*h.get("cpu-migrations").unwrap()),
            PageFaults : Some(*h.get("page-faults").unwrap()),
            Cycles : Some(*h.get("cycles").unwrap()),
            Instructions : Some(*h.get("instructions").unwrap()),
            Branches : Some(*h.get("branches").unwrap()),
            BranchMisses : Some(*h.get("branch-misses").unwrap()),
            Seconds : Some(*h.get("seconds").unwrap())
        }

    }
}
