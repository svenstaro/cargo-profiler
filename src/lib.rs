#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(type_ascription)]

extern crate regex;
extern crate clap;
extern crate itertools;
extern crate ndarray;

use std::process::Command;
use std::fmt;
use itertools::Zip;
use ndarray::{Axis, stack, OwnedArray, ArrayView, Ix};

pub type Mat<A> = OwnedArray<A,(Ix,Ix)>;

/// Profiler enum. We have three profilers: PerfStat, CacheGrind, and CallGrind.
pub enum Profiler<'a> {
    /// CachGrind holds the parsed objects of `valgrind --tool=cachegrind -cachegrind-out-file=cachegrind.out`
    CacheGrind {
        ir: Option<f64>,
        i1mr: Option<f64>,
        ilmr: Option<f64>,
        dr: Option<f64>,
        d1mr: Option<f64>,
        dlmr: Option<f64>,
        dw: Option<f64>,
        d1mw: Option<f64>,
        dlmw: Option<f64>,

        numbers : Option<Mat<f64>>,
        functs: Option<Vec<&'a str>>,
    },

    /// Call holds the parsed objects of `valgrind --tool=callgrind --callgrind-out-file=callgrind.out && cg_annotate callgrind.out`
    CallGrind {
        total_instructions: Option<f64>,
        instructions: Option<Vec<f64>>,
        functs: Option<Vec<&'a str>>,
    },
}

/// Pretty-print the profiler outputs into user-friendly formats.
impl<'a> fmt::Display for Profiler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {

            Profiler::CacheGrind { ref ir,
                                   ref i1mr,
                                   ref ilmr,
                                   ref dr,
                                   ref d1mr,
                                   ref dlmr,
                                   ref dw,
                                   ref d1mw,
                                   ref dlmw,

                                   ref numbers,
                                   ref functs } => {
                write!(f,
                       "\n\x1b[32mTotal Instructions\x1b[0m...{}\t\x1b[0m\n\n\
                       \x1b[32mTotal I1 Read Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal L1 Read Misses\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal D1 Reads\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal D1 Read Misses\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal DL1 Read Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal Writes\x1b[0m...{}\n\x1b[0m\
                       \x1b[32mTotal D1 Write Misses\x1b[0m...{}\t\x1b[0m\
                       \x1b[32mTotal DL1 Write Misses\x1b[0m...{}\x1b[0m\n\n\n",
                       ir.unwrap_or(std::f64::NAN),
                       i1mr.unwrap_or(std::f64::NAN),
                       ilmr.unwrap_or(std::f64::NAN),
                       dr.unwrap_or(std::f64::NAN),
                       d1mr.unwrap_or(std::f64::NAN),
                       dlmr.unwrap_or(std::f64::NAN),
                       dw.unwrap_or(std::f64::NAN),
                       d1mw.unwrap_or(std::f64::NAN),
                       dlmw.unwrap_or(std::f64::NAN),
                   );
                   write!(f, " \x1b[1;36mIr  \x1b[1;36mI1mr \x1b[1;36mILmr  \x1b[1;36mDr  \x1b[1;36mD1mr \x1b[1;36mDLmr  \x1b[1;36mDw  \x1b[1;36mD1mw \x1b[1;36mDLmw\n");
                   if let &Some(ref func) = functs {
                       if let &Some(ref ins) = numbers {
                           for (ref x, &y) in ins.axis_iter(Axis(0)).zip(func.iter()) {

                                   write!(f,
                                          "\x1b[0m{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} {}\n",
                                          x[0]/ir.unwrap() as f64, x[1]/i1mr.unwrap(),x[2]/ilmr.unwrap() as f64,x[3]/dr.unwrap() as f64,
                                          x[4]/d1mr.unwrap() as f64,x[5]/dlmr.unwrap() as f64,x[6]/dw.unwrap() as f64,
                                          x[7]/d1mw.unwrap() as f64,x[8]/dlmw.unwrap() as f64,
                                          y);
                                   println!("-----------------------------------------------------------------------");


                                   }
                               }
                           }




                   Ok(())

               }







            Profiler::CallGrind { ref total_instructions, ref instructions, ref functs } => {

                write!(f,
                       "\n\x1b[32mTotal Instructions\x1b[0m...{}\n\n\x1b[0m",
                       total_instructions.unwrap_or(std::f64::NAN));

                if let &Some(ref func) = functs {
                    if let &Some(ref ins) = instructions {
                        for (&x, &y) in ins.iter().zip(func.iter()) {
                            {

                                let perc = x / total_instructions.unwrap_or(std::f64::NAN) as f64 *
                                           100.;
                                match perc {
                                    t if t >= 50.0 => {
                                        write!(f,
                                               "{} (\x1b[31m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               t,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }
                                    t if (t >= 30.0) & (t < 50.0) => {
                                        write!(f,
                                               "{} (\x1b[33m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               t,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }
                                    _ => {
                                        write!(f,
                                               "{} (\x1b[32m{:.1}%\x1b[0m)\x1b[0m {}\n",
                                               x,
                                               x /
                                               total_instructions.unwrap_or(std::f64::NAN) as f64 *
                                               100.,
                                               y);
                                        println!("-----------------------------------------------------------------------");
                                    }

                                }
                            }
                        }
                    }
                }


                Ok(())

            }

        }




    }
}
/// Parser trait. To parse the output of Profilers, we first have to get their output from
/// the command line, and then parse the output into respective structs.
pub trait Parser {
    fn cli(&self, binary: &str) -> String;
    fn parse<'b>(&'b self, output: &'b str, n: &str) -> Profiler;
}

/// Initialize the Profilers
impl<'a> Profiler<'a> {
    /// Initialize CacheGrind
    pub fn new_cachegrind() -> Profiler<'a> {
        Profiler::CacheGrind {
            ir: None,
            i1mr: None,
            ilmr: None,
            dr: None,
            d1mr: None,
            dlmr: None,
            dw: None,
            d1mw: None,
            dlmw: None,

            numbers : None,
            functs: None,
        }
    }
    /// Initialize CallGrind
    pub fn new_callgrind() -> Profiler<'a> {
        Profiler::CallGrind {
            total_instructions: None,
            instructions: None,
            functs: None,
        }
    }
}


impl<'a> Parser for Profiler<'a> {
    /// Get profiler output from stdout.
    fn cli(&self, binary: &str) -> String {
        match *self {

            Profiler::CacheGrind { .. } => {
                Command::new("valgrind")
                    .arg("--tool=cachegrind")
                    .arg("--cachegrind-out-file=cachegrind.out")
                    .arg(binary)
                    .output()
                    .unwrap_or_else(|e| panic!("failed {}", e));
                let cachegrind_output = Command::new("cg_annotate")
                                            .arg("cachegrind.out")
                                            .arg(binary)
                                            .output()
                                            .unwrap_or_else(|e| panic!("failed {}", e));
                String::from_utf8(cachegrind_output.stdout).expect("cli error")
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

    /// Get parse the profiler output into respective structs.
    fn parse<'b>(&'b self, output: &'b str, n: &str) -> Profiler {
        match *self {

            Profiler::CacheGrind { .. } => {

                let out: Vec<&'b str> = output.split("\n").collect();
                let z = out[22..].to_owned();



                // let re = regex!(r"(\d+,\d{3},\d{3})|(\d+,\d{3})|[^\w\d{1}]&\d+(\.\d+)?|\d*\.\d+");
                // let re1 = regex!(r"[a-zA-Z\d{1}?]+\s*\t*[a-zA-Z\d{1}?]+\s*\t*[a-zA-Z]+:");

                let mut words: Vec<&str> = Vec::new();
                let mut numbers : Vec<Mat<f64>> = Vec :: new();

                for text in z.iter() {
                    let text = text.trim();
                    let mut elems = text.split(" ")
                                    .collect::<Vec<_>>();

                    elems.retain(|x| x.len() > 0);
                    if elems.len() > 7   {

                        let ns = elems[0..elems.len()-1].iter().map(|x| x.trim().replace(",","").parse::<f64>().unwrap()).collect::<Vec<f64>>();
                        if let Ok(e) = OwnedArray::from_shape_vec((ns.len(),1),ns){
                            numbers.push(e);
                        }



                        let path = elems[elems.len()-1].split(" ").collect::<Vec<_>>();
                        let sp = path[0].split("/").collect::<Vec<_>>();
                        words.push(sp[sp.len() - 1]);
                    }
                    }

            let mat = stack(Axis(1), &numbers.iter().map(|x| x.view()).collect::<Vec<_>>().as_slice()).ok().unwrap();
            let ir = mat.column(0).scalar_sum();
            let i1mr = mat.column(1).scalar_sum();
            let ilmr = mat.column(2).scalar_sum();
            let dr = mat.column(3).scalar_sum();
            let d1mr = mat.column(4).scalar_sum();
            let dlmr = mat.column(5).scalar_sum();
            let dw = mat.column(6).scalar_sum();
            let d1mw = mat.column(7).scalar_sum();
            let dlmw = mat.column(8).scalar_sum();



            if let Ok(s) = n.parse::<usize>() {
                words = words.iter().take(s).map(|x| x.to_owned()).collect();
            }

            Profiler::CacheGrind {
            ir: Some(ir),
            i1mr: Some(i1mr),
            ilmr: Some(ilmr),
            dr: Some(dr),
            d1mr: Some(d1mr),
            dlmr: Some(dlmr),
            dw: Some(dw),
            d1mw: Some(d1mw),
            dlmw: Some(dlmw),
            numbers : Some(mat),
            functs: Some(words),
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
                        let path = elems[1].split(" ").collect::<Vec<_>>();
                        let sp = path[0].split("/").collect::<Vec<_>>();
                        words.push(sp[sp.len() - 1])
                    }
                }

                let total_instructions = numbers.iter().fold(0.0, |a, b| a + b);
                if let Ok(s) = n.parse::<usize>() {
                    numbers = numbers.iter().take(s).map(|x| x.to_owned()).collect();
                    words = words.iter().take(s).map(|x| x.to_owned()).collect();
                }

                Profiler::CallGrind {
                    total_instructions: Some(total_instructions),
                    instructions: Some(numbers.iter().cloned().collect()),
                    functs: Some(words),
                }
            }

        }
    }
}
