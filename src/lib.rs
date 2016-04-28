#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;
use std::process::Command;
use std::collections::HashMap;

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
    seconds : Option<f64>
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
            seconds : None
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
        String::from_utf8(perf_stat_output.stderr).unwrap()

    }

    fn parse(&self, perf_stat_output : &str)-> Perf {

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
        for (&x,&y) in words.iter().zip(numbers.iter()) {
            h.insert(x,y);
        }

        Perf {
            task_clock : Some(*h.get("task-clock").unwrap()),
            context_switches : Some(*h.get("context-switches").unwrap()),
            cpu_migrations : Some(*h.get("cpu-migrations").unwrap()),
            page_faults : Some(*h.get("page-faults").unwrap()),
            cycles : Some(*h.get("cycles").unwrap()),
            instructions : Some(*h.get("instructions").unwrap()),
            branches : Some(*h.get("branches").unwrap()),
            branch_misses : Some(*h.get("branch-misses").unwrap()),
            seconds : Some(*h.get("seconds").unwrap())
        }

    }
}
